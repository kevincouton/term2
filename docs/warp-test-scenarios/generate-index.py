import re, glob, pathlib, datetime

files = sorted(glob.glob("warp-docs-chunk-*.md"))
total_scenarios = 0
all_areas = []

for f in files:
    text = pathlib.Path(f).read_text()
    scenarios = re.findall(r'^### Scenario:', text, re.M)
    total_scenarios += len(scenarios)
    areas = re.findall(r'^## (.+)$', text, re.M)
    all_areas.append((f, len(scenarios), areas))

with open("README.md", "w") as out:
    out.write("# Warp Test Scenarios Index\n\n")
    out.write(f"Extracted from https://docs.warp.dev/llms-full.txt on {datetime.datetime.now().isoformat()[:10]}.\n\n")
    out.write(f"Total scenarios: **{total_scenarios}** across {len(files)} chunk files.\n\n")
    out.write("## Files\n\n")
    for f, count, _ in all_areas:
        out.write(f"- [`{f}`](./{f}) — {count} scenarios\n")
    out.write("\n## Feature Areas by File\n\n")
    for f, count, areas in all_areas:
        out.write(f"\n### {f} ({count} scenarios)\n\n")
        for a in areas:
            out.write(f"- {a}\n")
