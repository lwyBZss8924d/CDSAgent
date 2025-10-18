# Issue-02: CDSAgent Technical Architecture Plan

Issue-02: CDSAgent Technical Architecture Plan frome @spacs/issues/01-CDSAgent-MVP-definition.md with @spacs/research/2025-10-18-cdsagent-requirement-analysis.md

- [x] TODO: Base "Issue-02: CDSAgent Technical Architecture Plan" analysis with "CDSAgent (Original Requirement Description)" and all {<REFERENCES_INDEX>} to design of the layered technical architecture PRDs files work Artifacts saved in (spacs/prd/)

## v0.2.0 TODO Items

### CDS-Index Core (from 02-index-core/, 10-extensibility.md)

- [ ] **Incremental Index Updates**: Define and implement graph index update triggers & mechanisms (detect repo edits, schedule incremental vs full rebuild) so CDSAgent-Index stays in sync with live codebases. Target: <500ms for single file updates. (PRD-10 §6.1, Issue 02-index-core/02-sparse-index.md)
- [ ] **Multi-Language Support**: Add TypeScript/JavaScript parsing (tree-sitter-typescript), Rust parsing (tree-sitter-rust). Implement `LanguageParser` trait per language. Validate parity with Python baseline accuracy. (PRD-01 §4.2, PRD-10 §6.3, Issue 02-index-core/01-graph-build.md)
- [ ] **LanceDB Evaluation**: Research unified storage for graph + vectors. Investigate if LanceDB can store heterogeneous graphs and combine BM25 + vector embeddings. Document decision (adopt/defer/reject). (PRD-10 §6.4, Issue 02-index-core/00-overview.md)
- [ ] **gRPC Service Promotion**: Promote gRPC module from experimental to stable for high-throughput scenarios. Pass all API contract tests, achieve latency <50ms (vs 100ms JSON-RPC). (PRD-02 §4.1, PRD-05 §9, PRD-10 §7.3)

### CDS-Tools CLI (from 03-cli-tools/, 10-extensibility.md)

- [ ] **MCP Tool Wrappers**: Implement native TypeScript MCP tools (`cds_search`, `cds_traverse`, `cds_retrieve`) calling Rust via RPC instead of bash. Target: Tool call latency <10ms (vs 50ms for bash). (PRD-04 FR-MCP-1, PRD-10 §6.2, Issue 04-agent-integration/00-overview.md)
- [ ] **cds combo Full Integration**: Promote `cds combo` from developer-only to full agent integration with YAML plan execution. Enable complex multi-step retrieval workflows for agent. (PRD-03 §2.1.4, Issue 03-cli-tools/00-overview.md)
- [ ] **Shell Completions**: Add bash, zsh, fish shell completions for improved developer experience. (PRD-03 §8, Issue 03-cli-tools/00-overview.md)
- [ ] **Colored Output & Progress Indicators**: Implement terminal colors and progress bars for better human usability. (PRD-03 §8, Issue 03-cli-tools/00-overview.md)

### CDS-Agent Integration (from 04-agent-integration/, 10-extensibility.md)

- [ ] **Multi-SDK Adapter Pattern**: Implement `CodexSDKAdapter` for OpenAI models. Ensure Claude → Codex swap works without CLI changes. Validate interface is SDK-agnostic. (PRD-10 §2, Issue 04-agent-integration/01-sdk-bootstrap.md, Issue 10-extensibility.md)
- [ ] **Native MCP Tools Migration**: Migrate from bash tool to native MCP tool wrappers for improved performance and type safety. Eliminate ~50ms bash overhead per tool call. (PRD-04 §2.4, PRD-10 §6.2, Issue 04-agent-integration/00-overview.md)
- [ ] **Fine-Tuning Pipeline**: Implement fine-tuning on successful agent trajectories (LocAgent §5.2 approach). Collect agent traces, fine-tune model, target Acc@5 improvement ≥10% over base model. (PRD-10 §8.1)

### Testing & Validation (from 08-testing/, 06-refactor-parity.md)

- [ ] **SWE-bench Full Validation**: Expand validation from Lite (300 instances) to full SWE-bench dataset. Target: File Acc@5 ≥80% maintained on larger set. (PRD-08 §5, Issue 08-testing/04-benchmark.md)
- [ ] **Continuous Parity Monitoring**: Set up automated parity tests that run on every LocAgent update. Alert if accuracy drops >5%. (Issue 06-refactor-parity.md)

### Extensibility & Future Features (from 10-extensibility.md)

- [ ] **Semantic/Vector Search**: Implement hybrid retrieval (BM25 + vector embeddings using CodeBERT). Store embeddings in vector DB (FAISS/LanceDB/Qdrant). Target: Improve recall@10 by ≥15%. (PRD-10 §7.1)
- [ ] **Go Language Support**: Add tree-sitter-go parser, test on Go repositories, validate accuracy. (PRD-01 §4.2, PRD-10 §7.2)
- [ ] **Real-Time Code Change Monitoring**: File watcher + auto-indexing daemon to keep index in sync with codebase edits. (PRD-10 §8.4)
- [ ] **Multi-Repo Federated Search**: Federated query router + index sharding for searching across microservices. (PRD-10 §8.2)
- [ ] **Web UI for Index Exploration**: Web interface for browsing graph, running searches, visualizing dependencies. (PRD-10 §8.3)

### Tracking References

- Primary Planning: [10-extensibility.md](./04-0.1.0-mvp/10-extensibility.md)
- Index Core: [02-index-core/00-overview.md](./04-0.1.0-mvp/02-index-core/00-overview.md)
- CLI Tools: [03-cli-tools/00-overview.md](./04-0.1.0-mvp/03-cli-tools/00-overview.md)
- Agent Integration: [04-agent-integration/00-overview.md](./04-0.1.0-mvp/04-agent-integration/00-overview.md)
- Parity Validation: [06-refactor-parity.md](./04-0.1.0-mvp/06-refactor-parity.md)

## CDSAgent Core Tech methods References from Paper & Repo !!! IMPORTANT

- CDSAgent build fllow LocAgent Paper: @tmp/LocAgent/arXiv-2503.09089v2
- LocAgent paper source: <https://arxiv.org/html/2503.09089v2>
- CDSAgent build fllow LocAgent Paper's Repo: @tmp/LocAgent/
- LocAgent paper's Repo source: <https://github.com/gersteinlab/LocAgent>

## CDSAgent Technical Architecture Plan

### Overview and Goals

CDSAgent (Codebase Fast DeepSearch Agent) is a code localization assistant that combines a graph-based code index with LLM agent capabilities for efficient code search. It builds on the LocAgent framework , using a directed heterogeneous code graph and sparse indices to bridge natural language issue descriptions with relevant code elements. The goal is to enable an LLM (Anthropic Claude 4.5 with a 1M token context) to autonomously navigate a codebase, performing multi-hop reasoning to find precise code locations for a given issue. Key objectives include:
 • Graph-Based Code Indexing: Parse the codebase into a lightweight graph of files, classes, functions, and their relationships (containment, imports, function calls, inheritance) . This provides structured context beyond raw text, so the agent can traverse code dependencies that aren’t obvious from file structure alone.
 • Hierarchical Search Indices: Maintain a two-tier Hierarchical Entity Index  – an upper index of entity names/IDs and a lower index of code content (BM25-based). This allows exact or fuzzy matching of keywords to code entities. LocAgent showed that such an index dramatically improves localization accuracy .
 • Unified Retrieval Tools: Expose three core operations as tools for the agent, unified in a CLI: SearchEntity, TraverseGraph, and RetrieveEntity  . These let the agent search for relevant code entities by keyword, traverse the code graph for n-hop dependencies, and retrieve full code for specific entities.
 • LLM Orchestration with Claude SDK: Use Anthropic’s Claude Agent SDK (TypeScript) to integrate the above tools into an agent workflow. The agent will plan steps (via chain-of-thought prompting as in LocAgent ) and invoke tools through the Model Context Protocol (MCP) for tool use . We will configure a specialized “code-retrieval” sub-agent with access to a sandboxed bash tool (and only allowed commands) to execute the cds CLI and related utilities.
 • Generality & Extensibility: Design the architecture to be LLM-agnostic where possible. While we start with Claude (streaming mode for interactive planning ), the components (index and tools) will be usable with other agent frameworks (OpenAI function calling, future GPT-4 Code or Gemini CLI, etc.) by swapping the agent integration layer. The CDS-Index service and CDS-Tools CLI are standalone, so any agent capable of running shell commands or API calls can leverage them.

Below we outline the component architecture and technical design in a linear task breakdown, with each major component as a ticket/issue. Each component includes reference-backed rationale and links to relevant code (from LocAgent) or SDK documentation as needed.

### 1. CDS-Index Service – Graph Indexer and Sparse Search Index

Task: Design and implement the code indexing pipeline that parses a given repository and builds the graph-based index and search structures.
 • Code Graph Construction (Rust): We will refactor LocAgent’s Python-based graph parser into a high-performance Rust module. The indexer will walk through the codebase (initially focusing on Python source, as in LocAgent). For each file, it uses an Abstract Syntax Tree parse to identify top-level classes and functions and their inner definitions recursively . Each entity (directory, file, class, function) becomes a node in the graph, with edges capturing: “contain” (directory contains file, file contains class/function), “import” (file imports file/module), “invoke” (function calls another function), and “inherit” (class subclasses another) . We assign each entity a unique ID (e.g., a fully-qualified name or hash) and record attributes like file path and line ranges. To enable multi-language support in the future, we will use Tree-sitter for AST parsing  – this supports many languages (Python, Java, C/C++, Go, etc.), making the approach extensible beyond Python with minimal changes .
 • Hierarchical Entity Index: Following LocAgent’s sparse hierarchical indexing design , the indexer creates two levels of indices:
 • Name/ID Index (Upper): a map from identifiers (entity names, fully-qualified names) to their entity IDs. This allows fast exact lookups when a query matches an entity’s name. For instance, a search for “AuthService” can directly find class or function names containing that keyword. If an exact or prefix match isn’t found at this level, the agent will fall back to the content index .
 • Content Index (Lower): a BM25 inverted index on the code content of entities (especially function and class bodies). We treat each function’s code as a document for retrieval , enabling keyword searches in code implementation text. This is built using a library like tantivy (Rust full-text search engine) or by integrating with the existing LocAgent BM25 index structure. LocAgent’s implementation (build_bm25_index.py) can be ported, or we can use an out-of-the-box BM25 from a search crate. The content index ensures that if a user’s query mentions terms not present in any identifier (e.g. an error message string or a variable name), the search can still find relevant code by content. This combination proved crucial: LocAgent’s ablation shows removing the SearchEntity tool or its BM25 index causes a significant drop in localization accuracy .
 • Graph Index Storage: The indexer will output the graph and indices to files so they can be loaded by the tools quickly. We can define a directory structure similar to LocAgent’s graph_index_v2.3 directory  containing:
 • A JSON or binary file for the graph (list of nodes and edges, or separate files per file/module).
 • The inverted index data for content search (e.g., serialized BM25 index).
 • An entity dictionary mapping names to IDs.
 • Optionally, precomputed folded code snippets for each entity (e.g., function signature only, which we’ll use for “fold” view results).
The indexing process can be run offline or on-demand. In practice, we might integrate it so that if an issue query comes for a repo not yet indexed, the agent first triggers this indexing (e.g., via a codebase-indexer sub-agent call). LocAgent batch-builds indexes in advance for all benchmark repos , but our design allows dynamic indexing. Indexing should be efficient: LocAgent notes it takes only a few seconds per codebase on average  due to the lightweight graph.
 • Integration with LLM (not immediate): Notably, building the index is mostly a deterministic parsing task. We do not require the LLM for this step, which is good for efficiency and determinism. However, we will ensure the indexer can be invoked via the agent if needed (for example, a tool call like /index-codebase could trigger it). The Anthropic SDK allows custom tools for such purposes via MCP . Initially, we might run the indexer manually or at session start, rather than having the agent spontaneously re-index. (In later iterations, the agent could detect code changes and update the index accordingly.)

Deliverables for this component: A Rust binary (and library) called cds-index that given a repository path (and optionally a config of which languages to parse) outputs the graph index files and search indices. We will also include tests (e.g., parse a sample repo and verify node counts, or that a known function can be found via keyword search). This service lays the groundwork for the next component, the query tools.

### 2. CDS-Retrieval Tools – Unified CLI for Code Search and Navigation

Task: Develop the CDS-Tools CLI (cds) providing commands for the agent (or a developer) to query the indexed code graph. This will refactor LocAgent’s internal tool APIs   into a standalone CLI with subcommands or flags. Each command corresponds to a fundamental operation the agent can perform on the codebase:
 • SearchEntity (cds -s or cds search): Given a keyword query, this tool searches the hierarchical index to find relevant code entities . It first checks the upper index for direct matches in entity names/IDs. If none (or not enough) are found, it performs a fuzzy/full-text search via the BM25 content index . For each match, it returns a structured snippet of the code:
 • Fold view: a brief one-liner representing the entity (e.g., function signature or class definition line) to give context without details .
 • Preview: a medium-length excerpt (perhaps the first ~5 lines of the function body or a summary) to show how the entity looks, avoiding dumping too much code.
 • Full code: the entire code of that entity (file, class, or function). Providing the full code is often unnecessary in early steps, so the agent might request it explicitly via RetrieveEntity later. By default, search can return the fold and/or preview, deferring full content for efficiency .
Each result also includes the entity’s ID, name, file path, and line number range. The output format can be JSON or a text block with clear separators (for easy parsing by the LLM). Rationale: This multi-level snippet approach prevents overwhelming the LLM context with large code dumps and reduces noise , while still giving enough info for the agent to decide relevancy.
 • TraverseGraph (cds -t or cds traverse): Performs a type-aware BFS on the code graph . The input is one or multiple starting entity IDs (e.g., those found via SearchEntity or provided by the agent). The tool then explores connected entities in the graph, with parameters to control the traversal:
 • Direction: forward, backward, or bidirectional (e.g., follow outgoing edges like function calls, or incoming edges like “what calls this function”). For example, forward from a function would find functions it calls (outgoing “invoke” edges), whereas backward would find who calls it (incoming “invoke” edges).
 • Hop limit: how many steps outward to traverse (to avoid infinite or overly broad walks).
 • Entity type filter: limit nodes to certain types (e.g. only classes and functions) .
 • Relation type filter: limit edge types (e.g. traverse only “contain” and “inherit” relations to see a structural hierarchy, or only “invoke” to follow call chains) .
The output is the subgraph of all entities reached. We will format this as an expanded tree structure , where the starting entity is the root and each level of indent represents a hop. We annotate each connection with the relation type and direction (perhaps using arrows or labels). For example:

```text
ClassA  (class) [ID 123]
 ├─ contains → func1()  (function) [ID 456]
 │    ├─ invokes → func2()  (function) [ID 789]
 │    └─ invokes → func3()  (function) [ID 1011]
 └─ inherits → BaseClass  (class) [ID 1314]
```

This tree-format encoding of the graph makes it easier for the LLM to perform reasoning on the structure . (Prior work shows that representing graphs as indented text improves LLM graph reasoning .) The TraverseGraph tool essentially gives the agent a “map” of the code relationships around certain points of interest, in one action. For instance, the agent can ask for “all functions called (directly or indirectly) by Function X up to 2 hops” or “the class inheritance hierarchy of Module Y” and get a succinct tree back. This greatly boosts efficiency compared to manually navigating file by file .

 • RetrieveEntity (cds -r or cds retrieve): Retrieves full details and content for one or more specified entity IDs . This is used when the agent has identified a particular function/file it deems important and now needs the entire code to analyze or provide as an answer. The tool will output the entity’s metadata (name, type, defined in which file, line span) and the full source code of that entity. For a function, we can include the docstring and body; for a class, possibly the whole class; for a file-level retrieval, the entire file content. This tool ensures that the agent can get complete context on demand, after narrowing down candidates via SearchEntity and TraverseGraph. It’s essentially a wrapper around looking up the index and reading the stored code from disk (or the original repo).

In addition to these primary commands, CDS-Tools will also support composing multiple operations via shell pipelines or combined flags. The user’s workflow suggests advanced usage like piping cds output into Unix tools (grep, jq, etc.) or chaining queries. For example, an agent might do: cds -s "XSS vulnerability" | cds -t --hops 1 --types function – i.e., search for “XSS vulnerability” to get an entry point function, then traverse one hop of function calls from those results. We will design cds output in a way that’s easy to pipe: e.g., a plain text list of IDs found by search, which can be piped into a traverse command (or we allow cds traverse --from-search "query" internally). Another scenario: combining with external tools – after cds search, the agent might pipe results to rg (ripgrep) for additional regex filtering, or use ast-grep to find a specific AST pattern in the returned code. Our CLI should play nicely with such usage:
 • Shell Integration: We will allow cds to accept either direct arguments or via STDIN when feasible. For instance, if cds search "keyword" outputs a JSON with found IDs, one could do cds search "keyword" | jq '.ids[]' | cds traverse -f - to traverse from those IDs (assuming -f - means read list of IDs from stdin).
 • While these complex pipelines might be overkill for manual use, they enable a “hybrid” retrieval strategy in the agent: using our structured tools plus traditional text search (grep) and semantic grep (ast-grep) in combination. This addresses cases where one method alone might miss something. The agent, being an LLM, can decide a strategy (e.g., first use SearchEntity, then refine with ripgrep on the files returned, etc.). We ensure the agent has permission to call rg, ast-grep, or similar commands through the bash tool (with careful allow-listing).
 • Implementation (Rust): All these subcommands will be implemented in Rust for efficiency on large codebases. Rust’s strong CLI ecosystem (e.g. using clap or structopt) will let us define subcommands and flags cleanly. The cds binary will read the index files produced by CDS-Index. For performance, it might memory-map the index or maintain an in-memory cache (if running as a long-lived process). Initially, each invocation of cds will load the necessary data, but we can optimize by running it as a service (e.g., a daemon process that keeps the index in memory and accepts queries via RPC). However, given the integration with an agent that can call shell commands, the simpler approach is to spawn cds on each use. We will keep the index on disk in an efficient format to minimize load time. If performance becomes an issue, we’ll consider an in-process MCP tool server so the agent can call Rust code directly (see below).
 • Accuracy and Testing: We will validate that the tools produce correct and useful outputs. For example, run cds search for known terms and ensure it returns expected entities (comparing against LocAgent’s results if possible). The BFS in traverse should be tested on small hand-made graph samples to ensure filtering works (e.g., traverse only “inherit” relations on a sample class hierarchy yields the correct tree). Since LocAgent’s code (in dependency_graph/ and repo_index/) already implements much of this logic (in Python), we will reference it during implementation:
 • The entity index and BM25 building logic can be adapted from build_bm25_index.py  and related util functions in LocAgent.
 • The graph traversal behavior is likely defined in LocAgent’s internal agent or plugin code – possibly in plugins/ or auto_search_main.py. We will check LocAgent’s repository for how it performs BFS and format output (Appendix A.1.2 of the paper describes alternatives; LocAgent chose a tree format ).
 • Output format: The paper’s Figure 7 and Appendix A.1.2 presumably show the chosen format. We’ll mirror that format for consistency, as it was empirically found to aid LLM reasoning .

Deliverables for this component: A Rust CLI tool cds with subcommands or flags for search, traverse, retrieve. Also, documentation (Markdown) for each command’s usage, and possibly a programmatic API (the Rust library can provide functions that we could expose via FFI if needed). The CLI will be packaged such that it can be installed on the agent’s host system (or included in a Docker container) for the LLM to invoke. This leads to how the LLM will use these tools.

### 3. LLM Agent Integration – Orchestrating with Claude and Tools

Task: Integrate the CDSAgent’s capabilities with an LLM (Claude 2/Claude 4.5 via AnthropIC’s Agent SDK) so that it can autonomously plan and execute the code search tasks. This involves configuring the agent’s prompts, tools, and possibly a specialized sub-agent.
 • Agent Environment Setup (Claude SDK): We will use the Anthropic Claude Agent SDK (TypeScript) to create an agent instance that has access to our custom tools. The Claude agent provides a framework with system prompts, message streaming, and a tool invocation mechanism (MCP)  . We will run the agent in streaming mode (interactive session) as recommended for complex multi-step tasks  – this allows the agent to produce intermediate thoughts, tool calls, and observe outputs incrementally (closer to a ReAct loop, rather than a single-shot monologue).
 • System Prompt and Chain-of-Thought: A custom system prompt will instruct the agent on its role and how to use the tools. We’ll incorporate LocAgent’s chain-of-thought approach  into this prompt, e.g.: “You are an autonomous code localization agent. You will be given a software issue description, and your job is to find the code snippets that need modifications. You have tools to search the codebase (SearchEntity), traverse the code dependency graph (TraverseGraph), and retrieve code (RetrieveEntity). Use them step-by-step: first extract keywords from the issue, then search for related entities, then follow code relations, and so on, as in the provided examples.” We will include definitions of each tool in the prompt (or via the SDK’s tool interface) with concise descriptions, essentially summarizing what they do (drawn from LocAgent’s Table 1  ). If needed, we can supply an example reasoning trace (from LocAgent’s successful trajectories) to nudge the agent towards effective planning.
 • Registering Tools via MCP: Claude’s SDK allows custom tool registration through the Model Context Protocol, meaning the agent can call external APIs or processes as tools . We have two integration strategies:

 1. Bash Interface (External Process): Leverage Claude’s built-in bash tool to run shell commands. We can simply allow the agent to use bash and then call our cds CLI (and other allowed CLI utilities) as needed. To make this safe, we’ll use the SDK’s permission model:
 • Set allowedTools to include bash (and possibly some file read tools if needed) .
 • Use a restrictive permissionMode and settings.json rules so the agent can only run certain commands via bash – effectively an allow-list of our cds commands and benign utilities. For example, disallow any bash calls with rm, network tools, or other destructive commands. We will allow cds -s/-t/-r, rg, ast-grep, jq, etc., as the user indicated, but even those can be prefixed with a special marker to avoid confusion. (In a secure deployment, we might create a wrapper script that validates input before executing cds.)
 • Claude Code’s permission system lets us deny or ask for confirmation on certain patterns . We’ll configure deny rules for anything outside our scope, and perhaps an interactive approval step for any unforeseen command.
Using the bash approach has the advantage of simplicity – no additional coding in the agent needed, since bash is a built-in tool. The agent’s outputs for tool use will include the stdout of our CLI, which we can capture.

 2. Native SDK Tool (In-Process): For performance and tighter integration, we can wrap cds functionality as a native tool. The Claude SDK supports defining an in-process tool function via their tool() helper or similar, which can run custom code directly in Node (or call out to Rust via FFI) . For instance, we could write a TypeScript function searchEntities(query: string): SearchResult[] that calls into a Rust library (using Neon or Wasm) or even spawns the cds binary and returns the parsed JSON result. By registering this as a tool, the agent could call it without invoking a shell. This would avoid overhead of spawning processes for each search, making tool calls faster (as noted by experienced users of the SDK ). We may start with the simpler bash approach, and later optimize to in-process MCP if performance demands (especially if the agent will call many tools in a single session).
 • Sub-agent for Code Execution: Anthropic’s agent platform allows defining sub-agents (sometimes called “subtasks” or slash-commands) that run with a different persona or specialized tools . We will set up a sub-agent (let’s call it "code-retrievaler") that is specifically configured for running code search commands. This sub-agent will have:
 • Its own system prompt focusing on code retrieval (“You are the Code Retrieval sub-agent, specialized in executing codebase queries and returning results to the main agent…”).
 • Only the bash tool (or our custom tools) enabled, with the allow-list we discussed. This isolation ensures that when the main agent “delegates” a task, the sub-agent can only perform code searches and not do anything outside its scope.
 • Possibly a larger context window model if needed (since results might be large). However, since Claude 4.5 already has a huge context, we might run both main and sub-agent on the same model. The sub-agent concept is still useful for modularity: e.g., the main agent could issue an instruction like /code_retrieval "Find all calls to function X" which triggers the sub-agent.
In practice, using a sub-agent might or might not be necessary. An alternative is a single agent with all tools. But the LocAgent approach was a single agent that plans everything itself, so we might emulate that with one agent thread. We will still structure the solution so that if parallelism or separation is needed, the sub-agent hook is ready. (Claude Code itself uses an “Architect” agent and a “Task” sub-agent for planning vs doing, as hinted in documentation .)
 • Tool Use Workflow: During a session, the agent will iterate through thought -> tool -> observation steps (ReAct style). For example:

 (1) Agent (thought): “The issue mentions XSS in user profile. Likely relates to input validation. I’ll search for sanitize.”
 (2) Agent triggers: bash: cds -s "sanitize input user profile" (this is a ToolUse block emitted by the agent).
 (3) CDS search runs and returns results (a few functions dealing with sanitization).
 (4) SDK: Captures the output and feeds it back as an observation (Claude’s Agent SDK wraps the stdout into an AssistantMessage with a special ToolResult content type).
 (5) Agent (thought): “Result shows a sanitize_html function in utils and a profile.py view function calling it. The profile view might be the entry point. I should see what that function does.”
Now agent might do:
 (6) Agent triggers: bash: cds -t -e function -r invoke -h 1 12345 (traverse 1 hop of “invoke” relations from the function ID found).
 (7) CDS traverse outputs a small call graph.
 (8) Agent: analyses it, decides next steps, maybe calls cds -r to retrieve full code of a suspicious function, etc.
 (9) Finally, the agent composes an answer with the relevant code snippets it gathered.
Throughout this, Claude’s SDK hooks can be used to fine-tune the process:
 • We will implement a PreToolUse hook to intercept the agent’s tool requests. For example, if the agent tries an unsupported command or malformed query, we can correct or block it (ensuring it stays on track). We might also log queries for debugging.
 • A PostToolUse hook will examine the output. This is useful to trim or format the tool results before the agent sees them. For instance, we might post-process the cds output to ensure it’s not too large: if full code is huge, maybe truncate or summarize it before feeding into the prompt (although ideally the agent only calls RetrieveEntity when necessary). We could also inject some annotation, e.g., numbering the results from SearchEntity for easier reference.
 • Subagent Stop hook could be used if we implement the sub-agent delegation: it can combine the final outcome of sub-agent back to main agent or clean up any state.
The SDK’s hook system and permission model will be configured for safety  . For instance, to avoid the agent outputting proprietary code inadvertently, we might have a hook check on the final answer content. The focus, however, is on enabling the agent to freely use the code search tools within the sandbox.
 • Claude Agent SDK References: We will follow best practices from the Claude SDK docs and community:
 • Use ClaudeAgentClient in Node to manage the session .
 • Set allowedTools to our tool list, permissionMode=default initially (meaning the agent must confirm file writes, but we won’t be doing writes) .
 • If needed, incorporate CLAUDE.md project memory (though in our case, the “memory” is mostly the code index which is external; CLAUDE.md might hold some static repo info like a summary of the project, but since we have structured search, this might not be necessary).
 • Ensure streaming responses so that tool blocks are handled in sequence rather than a giant batch output  .
 • We’ll utilize the MCP server feature to register our tools. This could involve running an SDK MCP server in Rust or Python. However, since our CLI is external, we might not need a persistent MCP server; we can simply declare a custom tool that calls a shell command.
 • Testing in Agent Loop: We will test the integrated system on sample issues (possibly from LocAgent’s Loc-Bench ). For a given issue description, we expect the agent to produce a series of tool calls. We’ll verify it can indeed call cds and get results, and that it ultimately finds the correct file/function for the fix (as per ground truth). This will likely require iterative prompt tuning (to get the agent to follow the 4-step CoT from LocAgent ) and adjustments in tool output formatting. LocAgent’s success with a fine-tuned model (Qwen) hints that few-shot or refined prompting can improve our agent’s reliability . In future, fine-tuning Claude or another model on successful traces could further boost performance, but initially we rely on prompting.

Deliverables for this component: A configured Claude agent (TypeScript code) that can be launched to handle queries. This includes the system prompt, tool definitions (either via bash or custom functions), and hook implementations. We will also provide a README/Notion documenting how to run the agent (e.g., setting the API key, running the Node script, ensuring cds CLI is installed). Additionally, example transcripts of the agent solving an issue will be given to demonstrate end-to-end behavior.

### 4. Extensibility and Future Integration

Task: Although not an immediate “ticket” for implementation, we design CDSAgent in a modular way to accommodate different environments and future improvements:
 • Support for Other LLM Providers: The architecture separating the tools from the agent means we could swap out Claude for a different LLM agent. For instance, using OpenAI GPT-4 with function calling: we could define OpenAI functions corresponding to search_entity, traverse_graph, retrieve_entity with the same behaviors. The function implementations on the server side would call our Rust library or CLI. The logic of the agent’s chain-of-thought would live in our code (or we prompt GPT-4 to follow a similar reasoning pattern). OpenAI’s Codex or function calling SDK could be plugged in place of Claude’s SDK with minimal changes to the tool layer. We have kept the interfaces simple (CLI with JSON in/out) so that any orchestrator can invoke them. This dual-tracking of artifacts (Claude agent setup vs a generic Markdown of tool specs) ensures we aren’t locked in to one platform.
 • Non-Interactive/Batch Usage: The components can be used in a non-interactive mode as well. For example, a script could take an issue description and directly call a sequence of cds commands (perhaps guided by some heuristic or smaller model) to produce a result. This could be useful for benchmarking or integration into CI systems. While the primary focus is the interactive agent, the underlying services (index and CLI) are general.
 • Scaling to Large Repos: As codebases grow, performance will be a concern. We have chosen Rust for core logic to maximize speed in parsing and searching. If needed, we can introduce caching layers. For example, if an agent session repeatedly queries the same repo, we could run a background CDS Indexer service that keeps the graph in memory and listens on a socket for queries (this would avoid re-loading index for each tool call). The Anthropic MCP architecture supports external tool servers for such scenarios . We could register our service so the agent calls it via RPC instead of bash. This is a future optimization; initially, per-call performance should be acceptable for reasonably sized repos (LocAgent handled thousands of files with sub-second query times using Python + BM25 , so Rust will be even faster).
 • Additional Tools: Down the road, we might add more specialized tools. For instance, a “semantic search” tool using embeddings (vector search) to complement BM25, or an “AST query” tool to find code patterns (though ast-grep via bash is already an option). The design can accommodate this by extending the cds CLI or adding new CLI commands. The agent can learn to choose these tools as needed (possibly guided by system prompt hints like “if keyword search fails, try semantic search”).
 • Continuous Learning: LocAgent improved using fine-tuning on successful agent trajectories . Our architecture supports iterative improvement: we can log all agent decisions and outcomes (via hooks and SDK logging) and later analyze them. This data could train a smaller model or even be used to refine prompts. Because our tools return structured outputs, we can also systematically evaluate correctness (e.g., did the final retrieved entity actually contain the bug fix as per a patch?). This ties into a potential evaluation harness similar to LocAgent’s evaluation scripts .

### Conclusion

In summary, the CDSAgent is designed as a layered system: **(1)** a Rust-based indexing backend that parses code into a graph and builds fast search indices <https://aclanthology.org/2025.acl-long.426.pdf#:~:text=Figure%202%3A%20Overview%20of%20LOCAGENT,combine%20the%20graph%20and%20tools>, **(2)** a set of robust CLI tools (cds) for code search/navigation that align with LocAgent’s proven techniques  <https://ar5iv.labs.arxiv.org/html/2503.09089#:~:text=SearchEntity%3A%20This%20tool%20searches%20codebases,reduces%20noise%20fed%20into%20agents><https://ar5iv.labs.arxiv.org/html/2503.09089#:~:text=RetreiveEntity%3A%20This%20tool%20retrieves%20complete,line%20number%2C%20and%20code%20content>, and **(3)** an LLM orchestration layer using Claude’s agent SDK to drive those tools in a reasoning loop. Each component is linked to reliable references – the LocAgent research paper & repo provide the blueprint for the indexing and tool logic, while Anthropic’s documentation guides the safe and effective integration of these capabilities into an AI agent <https://blog.getbind.co/2025/10/03/how-to-create-agents-with-claude-agents-sdk/#:~:text=,to%20attach%20custom%20external%20tools> <https://skywork.ai/blog/building-your-first-coding-agent-with-claude-code-sdk/#:~:text=,canUseTool%20callback>.

By breaking the design into these components and issues, we ensure that the project can be managed in Linear or GitHub Issues as a series of technical deliverables (graph indexer, search tools, Claude agent integration, etc.), each with clear reference points. The end result will be a **CDSAgent PRD** that not only meets the original requirements (fast, deep code search via an LLM) but is also maintainable, extensible, and grounded in state-of-the-art techniques for code intelligence. All source links and SDK references have been provided to facilitate implementation.

---

<REFERENCES_INDEX>

## REFERENCES INDEX

### LocAgent Paper

(LocAgent) Paper folder path: @tmp/LocAgent/arXiv-2503.09089v2/ :

```tree
CDSAgent main*​
❯ cd tmp/LocAgent/arXiv-2503.09089v2/

tmp/LocAgent/arXiv-2503.09089v2 main*​
❯ tree
.
├── 0_main.bbl
├── 0_main.tex
├── 1_intro.tex
├── 2503.09089v2.pdf
├── 2_related.tex
├── 3_method.tex
├── 4_dataset.tex
├── 5_experiment.tex
├── 6_1_appendix_details.tex
├── 6_2_appendix_graph.tex
├── 6_3_appendix_dataset_exp.tex
├── 6_4_appendix_prompt.tex
├── acl.sty
├── acl_natbib.bst
├── custom.bib
├── data
│   ├── ablation.tex
│   ├── api_detail.tex
│   ├── collect_dataset.tex
│   ├── cost_efficiency.tex
│   ├── dataset_dist.tex
│   ├── downstream_edit.tex
│   ├── eval_loc_bench.tex
│   ├── graph_comparison.tex
│   ├── main_acc.tex
│   ├── main_ndcg.tex
│   ├── main_precision.tex
│   ├── main_results.tex
│   ├── tab_api_lists.tex
│   ├── time_efficiency.tex
│   └── tree-structure.tex
└── fig
    ├── acc_5_among_hops.pdf
    ├── acc_5_among_hops_w_dist.pdf
    ├── acc_on_categories.pdf
    ├── acc_on_categories_loc_bench560.pdf
    ├── expanded_tree_format.pdf
    ├── fig_2_overview.pdf
    ├── figure1.pdf
    ├── file_acc_5_among_hops.png
    ├── fine-tune-qwen.png
    ├── fine_tune.pdf
    ├── func_acc_10_among_hops.png
    ├── intro1.png
    ├── keyword_entity.pdf
    ├── keywords_example.pdf
    ├── loc_agent_figure1.pdf
    ├── loc_agent_prompt.pdf
    ├── main_fig_v0.1.pdf
    ├── output_format.pdf
    ├── output_locbench.png
    ├── overview.pdf
    ├── preliminary_swe_lite.png
    ├── result_lite_file-.png
    ├── result_lite_file.png
    ├── result_lite_func-1.png
    └── result_lite_func.png

3 directories, 55 files
```

### LocAgent Paper's Repo

(LocAgent) Paper's Repo folder path: @tmp/LocAgent/ :

```tree
CDSAgent/tmp/LocAgent main*​
 ❯ tree
.
├── AGENTS.md
├── CLAUDE.md
├── CONTRIBUTING.md
├── LICENSE
├── README.md
├── arXiv-2503.09089v2/
├── assets
│   └── overview.png
├── auto_search_main.py
├── build_bm25_index.py
├── dependency_graph
│   ├── __init__.py
│   ├── batch_build_graph.py
│   ├── build_graph.py
│   └── traverse_graph.py
├── evaluation
│   ├── eval_metric.py
│   ├── loc_output
│   │   └── locagent
│   │       └── claude_3-5
│   │           └── loc_outputs.jsonl
│   └── run_evaluation.ipynb
├── plugins
│   ├── __init__.py
│   ├── location_tools
│   │   ├── __init__.py
│   │   ├── locationtools.py
│   │   ├── repo_ops
│   │   │   ├── __init__.py
│   │   │   └── repo_ops.py
│   │   ├── retriever
│   │   │   ├── __init__.py
│   │   │   ├── bm25_retriever.py
│   │   │   └── fuzzy_retriever.py
│   │   └── utils
│   │       ├── compress_file.py
│   │       ├── dependency.py
│   │       ├── result_format.py
│   │       └── util.py
│   └── requirement.py
├── repo_index
│   ├── __init__.py
│   ├── codeblocks
│   │   ├── __init__.py
│   │   ├── codeblocks.py
│   │   ├── module.py
│   │   └── parser
│   │       ├── __init__.py
│   │       ├── comment.py
│   │       ├── create.py
│   │       ├── java.py
│   │       ├── parser.py
│   │       ├── python.py
│   │       └── queries
│   │           ├── __init__.py
│   │           ├── java.scm
│   │           └── python.scm
│   ├── file_context.py
│   ├── index
│   │   ├── __init__.py
│   │   ├── code_index.py
│   │   ├── code_node.py
│   │   ├── embed_model.py
│   │   ├── epic_split.py
│   │   ├── settings.py
│   │   ├── simple_faiss.py
│   │   └── types.py
│   ├── repository.py
│   ├── types.py
│   ├── utils
│   │   ├── __init__.py
│   │   ├── colors.py
│   │   ├── repo.py
│   │   ├── tokenizer.py
│   │   └── xml.py
│   └── workspace.py
├── requirements.txt
├── scripts
│   ├── gen_bm25_index.sh
│   ├── gen_graph_index.sh
│   └── run.sh
├── sft_train.py
└── util
    ├── __init__.py
    ├── actions
    │   ├── action.py
    │   └── action_parser.py
    ├── benchmark
    │   ├── gen_oracle_locations.py
    │   ├── git_repo_manager.py
    │   ├── parse_patch.py
    │   ├── parse_python_file.py
    │   └── setup_repo.py
    ├── classify_issue.py
    ├── cost_analysis.py
    ├── extract_entity_from_issue.py
    ├── process_output.py
    ├── prompts
    │   ├── __init__.py
    │   ├── general_prompt.py
    │   ├── pipelines
    │   │   ├── auto_search_prompt.py
    │   │   └── simple_localize_pipeline.py
    │   ├── prompt.py
    │   ├── system_prompt.j2
    │   ├── user_prompt-.j2
    │   └── user_prompt.j2
    ├── runtime
    │   ├── content_tools.py
    │   ├── exceptions.py
    │   ├── execute_ipython.py
    │   ├── finish.py
    │   ├── fn_call_converter.py
    │   ├── function_calling.py
    │   └── structure_tools.py
    └── utils.py

28 directories, 146 files
```

### claude-agent-sdk dev-docs

`npm install @anthropic-ai/claude-agent-sdk`
TypeScript SDK Latest version: `0.1.22`

(claude-agent-sdk) dev-docs folder path: @tmp/claude-agent-sdk/ :

- @tmp/claude-agent-sdk/agent-sdk-typescript.md
- @tmp/claude-agent-sdk/guides-streaming-vs-single-mode.md
- @tmp/claude-agent-sdk/guides-permissions.md
- @tmp/claude-agent-sdk/guides-sessions.md
- @tmp/claude-agent-sdk/guides-hosting.md
- @tmp/claude-agent-sdk/guides-modifying-system-prompts.md
- @tmp/claude-agent-sdk/guides-mcp.md
- @tmp/claude-agent-sdk/guides-custom-tools.md
- @tmp/claude-agent-sdk/guides-subagents.md
- @tmp/claude-agent-sdk/guides-slash-commands.md
- @tmp/claude-agent-sdk/guides-cost-tracking.md
- @tmp/claude-agent-sdk/guides-todo-tracking.md

****claude-agent-sdk References****:

- claude-code-sdk-demos: @tmp/claude-agent-sdk/claude-code-sdk-demos/email-agent/

### Claude-Code-CLI dev-docs

(Claude-Code-CLI) dev-docs folder path: @tmp/claude-code-cli-docs/ :

```tree
CDSAgent/tmp/claude-code-cli-docs main*​
❯ tree
.
├── access-control-and-permissions.md
├── build-claude-code
│   ├── github-actions.md
│   ├── headless.md
│   ├── hooks-guide.md
│   ├── mcp.md
│   ├── plugins.md
│   ├── skills.md
│   ├── sub-agents.md
│   └── troubleshooting.md
├── config
│   ├── memory.md
│   ├── model-config.md
│   └── settings.md
├── deployment-claude-code
│   ├── devcontainer.md
│   ├── litellm-gateway.md
│   ├── network-config.md
│   └── overview.md
├── monitoring-usage.md
├── overview.md
├── plugin-marketplaces.md
├── reference
│   ├── checkpointing.md
│   ├── cli-reference.md
│   ├── hooks-reference.md
│   ├── interactive-mode.md
│   ├── plugins-reference.md
│   └── slash-commands.md
├── references-common-workflows.md
└── setup.md

5 directories, 27 files
```

</REFERENCES_INDEX>
