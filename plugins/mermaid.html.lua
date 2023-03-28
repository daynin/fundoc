function transform(text)
  -- [note] since there's no way to add this script though mdBook without downloading the file
  -- we can just import mermaid via CDN
  importMermaid = '<script type="module"> import mermaid from "https://cdn.jsdelivr.net/npm/mermaid@10.0.2/+esm"; mermaid.initialize({}); </script>'

  result = '<pre class="mermaid">' .. text .. '</pre>' .. importMermaid
end

