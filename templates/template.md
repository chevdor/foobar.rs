# {{ cl.repository.name }}

Number of changes: {{ cl.changes | length }}
{# {{ env | json_encode() }} #}

{%- if cl.changes -%}
{%- for c in cl.changes %}
- {{ c.number }}: {{ c.title }}
{%- endfor %}
{%- else -%}
No change found between {{ env.REF1 }} and {{ env.REF2 }}
{%- endif -%}
{% include "doc/junk.txt" %}
