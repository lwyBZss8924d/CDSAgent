#!/usr/bin/env python3
"""Debug script to compare node IDs between CDSAgent and LocAgent graphs"""

import pickle
import networkx as nx

# Load both graphs
print("Loading graphs...")
with open('.artifacts/spec-tasks-T-02-02-sparse-index/diag/graphs/graph_locagent_cdsagent.pkl', 'rb') as f:
    cdsagent_G = pickle.load(f)

with open('.artifacts/spec-tasks-T-02-02-sparse-index/diag/graphs/graph_locagent_locagent_golden.pkl', 'rb') as f:
    locagent_G = pickle.load(f)

print("\n=== CDSAgent Sample Nodes (first 10 functions) ===")
count = 0
for node, data in cdsagent_G.nodes(data=True):
    if data.get('type') == 'function' and count < 10:
        print(f"  {node}")
        count += 1

print("\n=== LocAgent Sample Nodes (first 10 functions) ===")
count = 0
for node, data in locagent_G.nodes(data=True):
    if data.get('type') == 'function' and count < 10:
        print(f"  {node}")
        count += 1

print("\n=== CDSAgent Sample Edges (first 5) ===")
for i, (u, v, data) in enumerate(list(cdsagent_G.edges(data=True))[:5]):
    print(f"  {data.get('type', 'unknown')}: {u} -> {v}")

print("\n=== LocAgent Sample Edges (first 5) ===")
for i, (u, v, data) in enumerate(list(locagent_G.edges(data=True))[:5]):
    print(f"  {data.get('type', 'unknown')}: {u} -> {v}")

# Check for specific function
test_func = "auto_search_main.py:auto_search_process"
print(f"\n=== Looking for '{test_func}' ===")
print(f"  In CDSAgent: {test_func in cdsagent_G}")
print(f"  In LocAgent: {test_func in locagent_G}")

# Find similar nodes
print("\n=== Nodes containing 'auto_search_process' ===")
print("CDSAgent:")
for node in cdsagent_G.nodes():
    if 'auto_search_process' in str(node):
        print(f"  {node}")
        break

print("LocAgent:")
for node in locagent_G.nodes():
    if 'auto_search_process' in str(node):
        print(f"  {node}")
        break
