" Convert python-style angle brackets to JS-style curly braces
/\<

" Convert `\\rank<3> foo` scaling style to `3: 'foo'` style
�kh/\\\\rank.\d

" Convert a python-style spell to a JS-style spell. Run each in order:
/Spell\(
/"""
o�ku/scaling
/    },

" Then 500 times:
/ritual_time =�kb\= 
�khvOo�klr�kr�kr�kb:�kr'�@7',�kd�kh
/rank:.*\n.*\/\/ target:
/\<
/\\\\area
/tat�kbgs: ..(attune|sustain)

" Fix incorrect "scaling" parsing:
/scaling: """

" fix attacks by referencing original target
/against (the|each) subject