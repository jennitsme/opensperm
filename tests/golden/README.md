# Golden transcripts

Deterministic fixtures for replay/testing.
- Store transcripts (inputs, tool calls, outputs) as JSON or YAML.
- Runner should validate against IPC schema and compare outputs deterministically.

Planned format:
```yaml
name: sample
steps:
  - request:
      type: tool_call_request
      id: 1
      tool: echo
      input: { text: "hi" }
    response:
      type: tool_call_response
      id: 1
      status: ok
      output: { text: "hi" }
```
