import sys

with open('openspec/changes/cql-multi-stage-pipeline-refactor/tasks.md', 'r') as f:
    text = f.read()

text = text.replace('- [ ] 3.30 Write integration tests', '- [x] 3.30 Write integration tests')
text = text.replace('- [ ] 3.31 Write integration tests', '- [x] 3.31 Write integration tests')

with open('openspec/changes/cql-multi-stage-pipeline-refactor/tasks.md', 'w') as f:
    f.write(text)

