function Transform(text)
  -- [note] since there's no way to add this script though mdBook without downloading the file
  -- we can just import mermaid via CDN
  ImportMermaid = '<script type="module"> import mermaid from "https://cdn.jsdelivr.net/npm/mermaid@10.0.2/+esm"; mermaid.initialize({}); </script>'

  Result = '<pre class="mermaid">' .. text .. '</pre>' .. ImportMermaid
end

