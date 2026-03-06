import sys

with open('openspec/changes/cql-multi-stage-pipeline-refactor/tasks.md', 'r') as f:
    text = f.read()

text = text.replace('- [x] 4.4 Create', '- [ ] 4.4 Create')
text = text.replace('- [x] 4.5 Create', '- [ ] 4.5 Create')
text = text.replace('- [x] 4.6 Create', '- [ ] 4.6 Create')
text = text.replace('- [x] 4.7 Create', '- [ ] 4.7 Create')

with open('openspec/changes/cql-multi-stage-pipeline-refactor/tasks.md', 'w') as f:
    f.write(text)

