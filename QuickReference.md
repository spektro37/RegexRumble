#Regex Quick Reference

| Element | Description | Example Pattern | Matching String | Non-matching String |
|---|---|---|---|---|
| x | Literal character - exact match of most non-special characters | abc | abc | xyz |
| . | Any character | a.c.e | axcye | aaaabcde |
| \d | Any digit 0-9 | abc12\d | abc129 | abc12z |
| \w | 'Word characters' | ab\w | abz | ab9 |
| \s | Most whitespace (tab, space, newline, etc.) | hi\smum | hi mum | himum |
| + | One or more of the previous character | x+yz | xxxxxyz | yz |
| * | Zero or more of the previous character | x+yz | yz | abc |
| {n} | Previous character exactly n times | x{3}yz | xxxyz | xyz |
| [xyz] | One of the characters in the list | [abc]99 | b99 | d99 |
| ^ | Start of string | ^abc | abc123 | 123abc |
