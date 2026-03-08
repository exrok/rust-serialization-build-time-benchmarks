import argparse
import json
import re
import pandas as pd
import seaborn as sns
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
from matplotlib.ticker import FuncFormatter, AutoMinorLocator, FixedLocator
import math

parser = argparse.ArgumentParser(description="Generate aggregate benchmark comparison chart")
parser.add_argument("--input", action="append", required=True, help="Input JSON result file (repeatable)")
parser.add_argument("--output", required=True, help="Output image file path (PNG)")
parser.add_argument("--title", default="Serialization Libraries Benchmark Comparison", help="Chart title")
args = parser.parse_args()

records = []
lib_versions = {}

for path in args.input:
    with open(path, "r") as f:
        for lib_name, crate_info, benchmarks in json.load(f):
            matching = [(name, ver) for name, ver in crate_info if lib_name in name]
            if matching:
                primary_crate = min(matching, key=lambda x: len(x[0]))
                lib_versions[lib_name] = primary_crate[1]

            for bench in benchmarks:
                bench_info = bench[0]
                metrics = bench[1]

                bench_type_key = list(bench_info.keys())[0]
                bench_type = re.sub(r'(?<=[a-z])(?=[A-Z])', ' ', bench_type_key)
                details = bench_info[bench_type_key]

                detail_str = ", ".join(f"{k}={v}" for k, v in details.items())
                variant_name = f"{bench_type} ({detail_str})"
                duration = metrics["duration"]["counter-value"]

                records.append({
                    "Library": lib_name,
                    "Type": bench_type,
                    "Variant": variant_name,
                    "Duration": duration,
                    "Source": path,
                })

df = pd.DataFrame(records)

baseline_df = df[df["Library"] == "baseline"][["Source", "Variant", "Duration"]].rename(columns={"Duration": "BaseDuration"})
df = df.merge(baseline_df, on=["Source", "Variant"], how="left")
df["BaseDuration"] = df["BaseDuration"].fillna(0)

df["AddedTime"] = (df["Duration"] - df["BaseDuration"]).clip(lower=1e-6)

df_libs = df[df["Library"] != "baseline"].copy()

min_durations = df_libs.groupby(["Source", "Variant"])["AddedTime"].transform("min")
df_libs["Score"] = df_libs["AddedTime"] / min_durations

lib_order = df_libs.groupby("Library")["Score"].median().sort_values().index.tolist()


font_family = "Iosevka ss09"
try:
    from matplotlib.font_manager import findfont, FontProperties
    findfont(FontProperties(family=font_family), fallback_to_default=False)
except ValueError:
    font_family = "monospace"

plt.style.use("dark_background")
sns.set_theme(style="darkgrid", context="talk", rc={
    "axes.facecolor": "#0D1117",
    "figure.facecolor": "#0D1117",
    "grid.color": "#222833",
    "text.color": "white",
    "axes.labelcolor": "grey",
    "xtick.color": "white",
    "ytick.color": "white",
    "axes.edgecolor": "#333",
    "font.family": font_family
})

num_libs = len(lib_order)
legend_height = 1.5
fig_height = num_libs * 0.86 + legend_height
fig, ax = plt.subplots(figsize=(14, fig_height))

unique_types = df_libs["Type"].unique()
type_colors = ['#E06C75', '#61AFEF', '#98C379', '#E5C07B', '#C678DD', '#56B6C2', '#D19A66']

color_map = {}
legend_patches = []

for i, btype in enumerate(unique_types):
    color = type_colors[i % len(type_colors)]
    variants = df_libs[df_libs["Type"] == btype]["Variant"].unique()

    for var in variants:
        color_map[var] = color

    legend_patches.append(mpatches.Patch(color=color, label=btype))


sns.stripplot(
    data=df_libs, x="Score", y="Library", hue="Variant",
    palette=color_map, jitter=True, size=12, alpha=0.8, ax=ax, zorder=3,
    order=lib_order
)

ax.set_ylim(num_libs -0.5, -0.5)

for i in range(num_libs):
    ax.axhspan(i - 0.4, i + 0.4, color='#1a1f2b', zorder=1)

ax.set_title(args.title, fontsize=22, pad=20, weight='bold')
ax.set_ylabel("", fontsize=0, weight='bold')
ax.set_xlabel("Relative Time Slower (Baseline Subtracted)", fontsize=16, weight='bold')

ytick_positions = ax.get_yticks()
ytick_labels = [label.get_text() for label in ax.get_yticklabels()]
ax.set_yticklabels([""] * len(ytick_labels))  # Clear default labels

for pos, label_text in zip(ytick_positions, ytick_labels):
    if label_text in lib_versions:
        ax.annotate(label_text, xy=(-0.02, pos), xycoords=ax.get_yaxis_transform(),
                    xytext=(0, 3), textcoords='offset points',
                    ha='right', va='bottom', fontsize=15, fontweight='bold', color='white')
        ax.annotate(f"v{lib_versions[label_text]}", xy=(-0.02, pos), xycoords=ax.get_yaxis_transform(),
                    xytext=(0, -3), textcoords='offset points',
                    ha='right', va='top', fontsize=15, color='grey')
    else:
        ax.text(-0.02, pos, label_text, transform=ax.get_yaxis_transform(),
                ha='right', va='center', fontsize=13, fontweight='bold', color='white')
ax.set_xscale('linear')

ax.set_xlim(left=0.5)
xmax_data = df_libs["Score"].max()
step = max(1, math.ceil(xmax_data / 10))
major_ticks = list(range(1, math.ceil(xmax_data) + step + 1, step))
ax.xaxis.set_major_locator(FixedLocator(major_ticks))

ax.xaxis.set_major_formatter(FuncFormatter(lambda x, pos: f"{x:g}x"))

ax.xaxis.set_minor_locator(AutoMinorLocator(2))

ax.grid(False)
xmin, xmax = ax.get_xlim()
for tick in major_ticks:
    if xmin <= tick <= xmax:
        ax.axvline(tick, color="#444444", alpha=0.55, linestyle="-", linewidth=1.2, zorder=2)
for tick in ax.xaxis.get_minorticklocs():
    if tick >= 1 and xmin <= tick <= xmax:
        ax.axvline(tick, color="#444444", alpha=0.50, linestyle="--", linewidth=0.8, zorder=2)
ax.set_xlim(xmin, xmax)

if ax.legend_:
    ax.legend_.remove()

fig.legend(
    handles=legend_patches,
    title="",
    loc="lower center",
    bbox_to_anchor=(0.5, 0.02),
    ncol=len(unique_types),
    frameon=False
)

plt.subplots_adjust(bottom=legend_height / fig_height)

plt.savefig(args.output, dpi=150, bbox_inches='tight', facecolor=fig.get_facecolor())
plt.close()
