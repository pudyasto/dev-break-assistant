<template>
  <div class="markdown-content" v-html="html" />
</template>

<script setup lang="ts">
import { computed } from 'vue'
import DOMPurify from 'dompurify'
import { marked } from 'marked'

const props = defineProps<{ content: string }>()

/**
 * Ollama responses are Markdown. Convert them here rather than rendering the
 * response as plain text, then sanitize the generated HTML before it reaches
 * the DOM. `++text++` is also accepted as a convenient underline extension.
 */
const html = computed(() => {
  const source = props.content.replace(/(?<!\\)\+\+([\s\S]+?)\+\+/g, '<u>$1</u>')
  return DOMPurify.sanitize(marked.parse(source, { gfm: true, breaks: true }) as string)
})
</script>

<style scoped>
.markdown-content :deep(p),
.markdown-content :deep(ul),
.markdown-content :deep(ol),
.markdown-content :deep(blockquote),
.markdown-content :deep(pre) {
  margin: 0.5rem 0;
}

.markdown-content :deep(:first-child) { margin-top: 0; }
.markdown-content :deep(:last-child) { margin-bottom: 0; }
.markdown-content :deep(h1),
.markdown-content :deep(h2),
.markdown-content :deep(h3),
.markdown-content :deep(h4) {
  margin: 0.75rem 0 0.4rem;
  font-weight: 700;
  line-height: 1.25;
}
.markdown-content :deep(h1) { font-size: 1.35em; }
.markdown-content :deep(h2) { font-size: 1.2em; }
.markdown-content :deep(h3) { font-size: 1.1em; }
.markdown-content :deep(ul),
.markdown-content :deep(ol) { padding-left: 1.25rem; }
.markdown-content :deep(ul) { list-style: disc; }
.markdown-content :deep(ol) { list-style: decimal; }
.markdown-content :deep(strong) { font-weight: 700; }
.markdown-content :deep(em) { font-style: italic; }
.markdown-content :deep(u) { text-decoration: underline; }
.markdown-content :deep(s) { text-decoration: line-through; }
.markdown-content :deep(code) {
  padding: 0.1rem 0.3rem;
  border-radius: 0.25rem;
  background: color-mix(in srgb, currentColor 12%, transparent);
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
}
.markdown-content :deep(pre) {
  overflow-x: auto;
  padding: 0.75rem;
  border-radius: 0.5rem;
  background: color-mix(in srgb, currentColor 8%, transparent);
}
.markdown-content :deep(pre code) { padding: 0; background: transparent; }
.markdown-content :deep(a) { color: var(--color-accent); text-decoration: underline; }
.markdown-content :deep(blockquote) {
  border-left: 3px solid var(--color-accent);
  padding-left: 0.75rem;
  opacity: 0.85;
}
.markdown-content :deep(hr) { margin: 0.75rem 0; border-color: var(--color-border); }
.markdown-content :deep(table) { width: 100%; margin: 0.5rem 0; border-collapse: collapse; }
.markdown-content :deep(th),
.markdown-content :deep(td) { padding: 0.35rem; border: 1px solid var(--color-border); text-align: left; }
.markdown-content :deep(th) { font-weight: 700; }
</style>
