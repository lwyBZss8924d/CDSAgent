# PRD-10: Extensibility & Future Enhancements

**Version:** 1.0 (Round 1 - Concise)
**Date:** 2025-10-18
**Status:** Draft
**Parent:** PRD-01 System Architecture

---

## 1. Document Overview

### 1.1 Purpose

Define extensibility mechanisms and future enhancement roadmap to ensure CDSAgent remains adaptable to new LLM providers, programming languages, search techniques, and research developments beyond v1.0.

### 1.2 Scope

- Multi-SDK abstraction layer (Claude, OpenAI Codex, Gemini, etc.)
- Multi-language code parsing extensibility
- Alternative search strategies (vector/semantic, hybrid)
- Research-driven improvements (fine-tuning, continuous learning)
- Integration with other developer tools

---

## 2. Multi-SDK Support

### 2.1 Objective

Enable CDSAgent to work with multiple LLM agent SDKs without changing core retrieval components (CDS-Index, CDS-Tools).

### 2.2 Agent SDK Abstraction Layer

**Interface Design**:

```typescript
// cds-agent/src/sdk-adapter.ts

export interface AgentSDKAdapter {
  // Initialization
  initialize(config: SDKConfig): Promise<void>;

  // Agent interaction
  query(userMessage: string, options?: QueryOptions): Promise<AgentResponse>;

  // Tool registration
  registerTool(tool: ToolDefinition): void;
  registerTools(tools: ToolDefinition[]): void;

  // Hooks
  setHooks(hooks: AgentHooks): void;

  // Session management
  getSessionId(): string;
  resetSession(): void;
}

export interface SDKConfig {
  apiKey: string;
  model: string;
  systemPrompt: string;
  maxTokens?: number;
  temperature?: number;
}

export interface ToolDefinition {
  name: string;
  description: string;
  parameters: Record<string, any>;
  execute: (params: any) => Promise<any>;
}

export interface AgentHooks {
  preToolUse?: (context: ToolContext) => Promise<HookResult>;
  postToolUse?: (context: ToolContext) => Promise<HookResult>;
  onError?: (error: Error) => Promise<void>;
}
```

### 2.3 SDK Implementations

#### Claude SDK Adapter (v1.0)

```typescript
// cds-agent/src/adapters/claude-adapter.ts

import { ClaudeAgentClient } from '@anthropic-ai/claude-agent-sdk';

export class ClaudeSDKAdapter implements AgentSDKAdapter {
  private client: ClaudeAgentClient;

  async initialize(config: SDKConfig): Promise<void> {
    this.client = new ClaudeAgentClient({
      apiKey: config.apiKey,
      model: config.model,
      systemPrompt: config.systemPrompt,
      mode: 'streaming',
      // ...
    });
  }

  async query(userMessage: string, options?: QueryOptions): Promise<AgentResponse> {
    const response = await this.client.query({
      message: userMessage,
      maxIterations: options?.maxIterations || 10,
    });

    return {
      finalAnswer: response.output,
      toolCalls: response.toolCalls,
      reasoning: response.thoughts,
    };
  }

  // ... other methods
}
```

#### OpenAI Codex Adapter (v1.1)

```typescript
// cds-agent/src/adapters/codex-adapter.ts

import { OpenAI } from 'openai';

export class CodexSDKAdapter implements AgentSDKAdapter {
  private client: OpenAI;

  async initialize(config: SDKConfig): Promise<void> {
    this.client = new OpenAI({ apiKey: config.apiKey });
  }

  async query(userMessage: string, options?: QueryOptions): Promise<AgentResponse> {
    // Use OpenAI function calling for tools
    const messages = [
      { role: 'system', content: config.systemPrompt },
      { role: 'user', content: userMessage }
    ];

    const response = await this.client.chat.completions.create({
      model: config.model, // e.g., 'gpt-4-turbo'
      messages,
      functions: this.registeredTools,
      function_call: 'auto',
    });

    // Handle function calls, iterate...
    return this.processFunctionCalls(response);
  }

  // ... other methods
}
```

#### Gemini Adapter (v1.2)

```typescript
// cds-agent/src/adapters/gemini-adapter.ts

import { GoogleGenerativeAI } from '@google/generative-ai';

export class GeminiSDKAdapter implements AgentSDKAdapter {
  // Similar structure, use Gemini function calling
}
```

### 2.4 Adapter Factory

```typescript
// cds-agent/src/agent-factory.ts

export function createAgent(
  sdkType: 'claude' | 'codex' | 'gemini',
  config: SDKConfig
): AgentSDKAdapter {
  switch (sdkType) {
    case 'claude':
      return new ClaudeSDKAdapter();
    case 'codex':
      return new CodexSDKAdapter();
    case 'gemini':
      return new GeminiSDKAdapter();
    default:
      throw new Error(`Unknown SDK type: ${sdkType}`);
  }
}

// Usage:
const agent = createAgent('claude', {
  apiKey: process.env.ANTHROPIC_API_KEY,
  model: 'claude-sonnet-4-5',
  systemPrompt: loadSystemPrompt(),
});

await agent.initialize();
const result = await agent.query('Find XSS vulnerability in user profile');
```

### 2.5 Acceptance Criteria

- [ ] Can swap SDK via config parameter
- [ ] All core functionality (search, traverse, retrieve) works across SDKs
- [ ] Tool definitions portable between SDKs
- [ ] System prompts reusable (or easily adaptable)

---

## 3. Multi-Language Code Parsing

### 3.1 Objective

Support indexing and searching code in languages beyond Python (TypeScript, Java, C++, Go, Rust, etc.).

### 3.2 Language Plugin Architecture

**Abstract Parser Interface**:

```rust
// cds_graph/src/language_parser.rs

pub trait LanguageParser {
    /// Language name (e.g., "python", "typescript")
    fn language_name(&self) -> &str;

    /// File extensions this parser handles (e.g., [".py", ".pyw"])
    fn file_extensions(&self) -> &[&str];

    /// Parse source code and extract entities
    fn parse(&self, source: &str, file_path: &Path) -> Result<Vec<Entity>>;

    /// Extract edges (imports, invokes, inherits) from parsed entities
    fn extract_edges(&self, entities: &[Entity], source: &str) -> Result<Vec<Edge>>;
}
```

**Language Registry**:

```rust
// cds_graph/src/registry.rs

pub struct ParserRegistry {
    parsers: HashMap<String, Box<dyn LanguageParser>>,
}

impl ParserRegistry {
    pub fn new() -> Self {
        let mut registry = Self { parsers: HashMap::new() };

        // Register built-in parsers
        registry.register(Box::new(PythonParser::new()));
        registry.register(Box::new(TypeScriptParser::new()));
        registry.register(Box::new(JavaParser::new()));

        registry
    }

    pub fn register(&mut self, parser: Box<dyn LanguageParser>) {
        self.parsers.insert(parser.language_name().to_string(), parser);
    }

    pub fn get_parser_for_file(&self, file_path: &Path) -> Option<&dyn LanguageParser> {
        let ext = file_path.extension()?.to_str()?;
        self.parsers.values().find(|p| p.file_extensions().contains(&ext))
    }
}
```

**Example: TypeScript Parser**:

```rust
// cds_graph/src/parsers/typescript.rs

use tree_sitter::{Language, Query};

extern "C" { fn tree_sitter_typescript() -> Language; }

pub struct TypeScriptParser {
    language: Language,
    query: Query,
}

impl LanguageParser for TypeScriptParser {
    fn language_name(&self) -> &str { "typescript" }

    fn file_extensions(&self) -> &[&str] { &[".ts", ".tsx"] }

    fn parse(&self, source: &str, file_path: &Path) -> Result<Vec<Entity>> {
        let mut parser = Parser::new();
        parser.set_language(self.language)?;

        let tree = parser.parse(source, None).ok_or(ParseError)?;
        self.extract_entities(&tree, source, file_path)
    }

    // ... similar to Python parser but with TS-specific queries
}
```

### 3.3 Adding New Languages (Plugin System)

**External Language Plugins** (future):

```rust
// cds_graph_plugin_cpp/src/lib.rs

#[no_mangle]
pub extern "C" fn create_parser() -> Box<dyn LanguageParser> {
    Box::new(CppParser::new())
}
```

**Load plugins dynamically**:

```rust
// Load C++ parser plugin at runtime
let plugin = libloading::Library::new("libcds_graph_plugin_cpp.so")?;
let create_parser: libloading::Symbol<fn() -> Box<dyn LanguageParser>> =
    unsafe { plugin.get(b"create_parser")? };

let cpp_parser = create_parser();
registry.register(cpp_parser);
```

### 3.4 Language Support Roadmap

| Language | Version | Priority | Tree-sitter Grammar | Effort |
|----------|---------|----------|-------------------|--------|
| Python | v1.0 | Critical | ✅ Available | Complete |
| TypeScript/JavaScript | v1.1 | High | ✅ Available | 1 week |
| Java | v1.1 | High | ✅ Available | 1 week |
| C/C++ | v1.2 | Medium | ✅ Available | 2 weeks |
| Go | v1.2 | Medium | ✅ Available | 1 week |
| Rust | v1.3 | Low | ✅ Available | 1 week |
| Ruby | v1.3 | Low | ✅ Available | 1 week |

### 3.5 Acceptance Criteria

- [ ] Parser registry allows adding languages without modifying core
- [ ] TypeScript and Java parsers functional (v1.1)
- [ ] Can index multi-language repositories (e.g., Python backend + TS frontend)

---

## 4. Alternative Search Strategies

### 4.1 Vector/Semantic Search

**Objective**: Complement BM25 sparse retrieval with dense vector embeddings for semantic matching.

**Architecture**:

```rust
// cds_semantic_index/src/lib.rs

use ndarray::Array2;

pub struct SemanticIndex {
    embeddings: Array2<f32>,         // N x D matrix (N entities, D dimensions)
    entity_ids: Vec<String>,
    model: EmbeddingModel,           // e.g., CodeBERT, UniXcoder
}

impl SemanticIndex {
    pub fn build(entities: &[Entity]) -> Result<Self> {
        let model = EmbeddingModel::load("microsoft/unixcoder-base")?;

        let embeddings: Vec<Vec<f32>> = entities
            .iter()
            .map(|e| model.encode(&e.code))
            .collect();

        // Convert to matrix, build FAISS index...
    }

    pub fn search(&self, query: &str, k: usize) -> Vec<(String, f32)> {
        let query_emb = self.model.encode(query);

        // Cosine similarity search
        let scores: Vec<f32> = self.embeddings
            .outer_iter()
            .map(|emb| cosine_similarity(&query_emb, emb.as_slice().unwrap()))
            .collect();

        // Return top-k
        top_k_indices(&scores, k)
            .into_iter()
            .map(|i| (self.entity_ids[i].clone(), scores[i]))
            .collect()
    }
}
```

**Hybrid Search** (BM25 + Vector):

```rust
pub fn hybrid_search(query: &str, limit: usize) -> Vec<SearchResult> {
    let bm25_results = bm25_index.search(query, limit);
    let vector_results = semantic_index.search(query, limit);

    // Combine with weighted scores
    let combined = merge_results(bm25_results, vector_results, alpha=0.7);
    combined[..limit]
}
```

**Roadmap**:

- v1.2: Prototype semantic search with CodeBERT
- v1.3: Evaluate hybrid search on SWE-bench (expect +5-10% accuracy)
- v1.4: Production-ready semantic index

### 4.2 LLM-Based Re-ranking

**Objective**: Use LLM to re-rank BM25/vector search results based on semantic relevance.

**Flow**:

1. BM25 search returns top-50 candidates
2. LLM (small model like GPT-3.5) scores each candidate: "How relevant is this code to query X?"
3. Re-rank by LLM scores
4. Return top-10

**Implementation**:

```typescript
async function rerankResults(query: string, candidates: SearchResult[]): Promise<SearchResult[]> {
  const scores = await Promise.all(
    candidates.map(async (result) => {
      const prompt = `
Query: "${query}"
Code:
\`\`\`
${result.snippet.preview}
\`\`\`

Relevance score (0-10):`;

      const response = await llm.generate(prompt);
      const score = parseFloat(response);
      return { ...result, score };
    })
  );

  return scores.sort((a, b) => b.score - a.score);
}
```

---

## 5. Continuous Learning & Fine-Tuning

### 5.1 Agent Trajectory Collection

**Objective**: Log all agent sessions for future fine-tuning (as in LocAgent §5.2).

**Session Log Format**:

```json
{
  "session_id": "abc123",
  "timestamp": "2025-10-18T10:30:00Z",
  "issue": "Find XSS vulnerability in user profile",
  "tool_calls": [
    {"tool": "cds search", "input": "sanitize XSS", "output": "[...]"},
    {"tool": "cds traverse", "input": "entity_123", "output": "[...]"}
  ],
  "final_answer": {
    "locations": [{"file": "views.py", "line_range": [45, 60]}]
  },
  "ground_truth": {
    "locations": [{"file": "views.py", "line_range": [45, 60]}]
  },
  "success": true
}
```

**Storage**: Append to `spacs/research/agent-sessions.jsonl`

### 5.2 Fine-Tuning Pipeline (v1.2)

**Steps**:

1. Collect 500+ successful agent trajectories
2. Format as instruction-tuning dataset:

   ```json
   {
     "instruction": "Find code related to XSS in user profile",
     "input": "Repository index available. Tools: search, traverse, retrieve.",
     "output": "Step 1: cds search \"XSS sanitize user profile\"\nStep 2: cds traverse entity_abc --depth 1\n..."
   }
   ```

3. Fine-tune model (e.g., Qwen-2.5-Coder as in LocAgent)
4. Evaluate: Compare fine-tuned vs base model on held-out SWE-bench

**Expected Impact**: +10-15% accuracy (based on LocAgent results)

### 5.3 Active Learning

**Objective**: Agent requests human feedback on uncertain predictions.

**Flow**:

1. Agent outputs low-confidence answer (e.g., ambiguous search results)
2. Present top-3 options to user: "Which location is most relevant?"
3. User selects correct option
4. Log feedback for fine-tuning

**Implementation**:

```typescript
if (confidence < 0.5) {
  const feedback = await askUser({
    question: 'Which location is most relevant?',
    options: topLocations,
  });

  logFeedback(sessionId, feedback);
}
```

---

## 6. Integration with Developer Tools

### 6.1 IDE Extensions

**Objective**: Integrate CDSAgent into VSCode, IntelliJ, etc.

**VSCode Extension** (v2.0):

- Command: "CDSAgent: Search Code"
- Right-click context menu: "Find Related Code"
- Sidebar panel: Display traversal graph

**Implementation**:

- Extension calls CDS-Index Service (localhost)
- Display results in VSCode panel
- Jump to code location on click

### 6.2 GitHub Actions Integration

**Objective**: Auto-locate relevant code for new issues/PRs.

**Workflow**:

```yaml
name: CDSAgent Issue Triage

on:
  issues:
    types: [opened]

jobs:
  locate-code:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run CDSAgent
        run: |
          cds-agent --issue "${{ github.event.issue.body }}" --output locations.json

      - name: Comment on issue
        uses: actions/github-script@v6
        with:
          script: |
            const locations = require('./locations.json');
            const comment = `Relevant code locations:\n${locations.map(l => `- ${l.file}:${l.line_range[0]}`).join('\n')}`;
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              body: comment
            });
```

### 6.3 Slack/Discord Bot

**Objective**: Query codebase via chat.

**Example**:

```text
User: @cdsbot find auth code
Bot: Found 5 locations:
     1. auth/login.py:45-60
     2. middleware/auth.ts:12-34
     [View in GitHub] [Run deeper search]
```

---

## 7. Research-Driven Enhancements

### 7.1 Graph Neural Networks for Code

**Objective**: Use GNNs to learn better code representations.

**Approach**:

- Represent code graph as GNN input
- Train GNN to predict "which nodes are relevant to a query?"
- Use GNN scores to augment BM25/vector search

**Timeline**: Research prototype (v1.4+)

### 7.2 Neuro-Symbolic Reasoning

**Objective**: Combine LLM reasoning with symbolic graph traversal.

**Approach**:

- LLM generates traversal plan (symbolic query)
- Graph engine executes plan deterministically
- LLM interprets results

**Benefit**: More reliable, explainable agent behavior

### 7.3 Multi-Modal Code Understanding

**Objective**: Incorporate code diagrams, screenshots, documentation.

**Example**: If issue mentions "the login flow diagram," retrieve related images and code together.

---

## 8. Extensibility Acceptance Criteria

### Must-Have (v1.0)

- [ ] CLI-first design allows any agent SDK to use CDS-Tools
- [ ] Parser registry supports adding languages

### Should-Have (v1.1-v1.2)

- [ ] Multi-SDK abstraction layer implemented
- [ ] TypeScript and Java parsers added
- [ ] Semantic search prototype validated

### Nice-to-Have (v1.3+)

- [ ] Fine-tuned model outperforms base model
- [ ] IDE extension MVP
- [ ] Active learning feedback loop

---

## 9. Open Questions

1. **Vector Index Size**: How to scale semantic index to 100K+ files? (Use approximate nearest neighbors, FAISS)
2. **Fine-Tuning Cost**: Is fine-tuning worth the effort vs prompt engineering? (Validate with pilot)
3. **Multi-Repo**: Federated search across repos—how to handle entity ID collisions? (Namespace by repo)

---

## 10. Long-Term Vision (v2.0+)

### Code Intelligence Platform

- **CDSAgent Core**: Code localization (v1.0)
- **CDS-Edit**: Automated code patching (integrate with Aider, SWE-agent)
- **CDS-Review**: PR review assistant (find related tests, check for regressions)
- **CDS-Explain**: Natural language code explanations
- **CDS-Docs**: Auto-generate documentation from code

**Goal**: Unified platform for AI-assisted software engineering, grounded in LocAgent research.

---

**Status**: Ready for v1.0 focus. Future enhancements to be prioritized post-launch based on user feedback and research developments.

**Next Steps**:

1. Implement core v1.0 features (PRD-01 to PRD-09)
2. Validate extensibility design with pilot multi-SDK test
3. Collect user feedback on most-wanted features for v1.1+
