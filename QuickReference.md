#Regex Quick Reference

| Element | Description | Example Pattern | Matching String | Non-matching String |
|---|---|---|---|---|
| x | Literal character - exact match of most non-special characters | abc | abc | xyz |
| . | Any character | a.c.e | axcye | abbbcde |
| \w | 'Word characters' - alphanumeric or underscore | ab\w | abz | ab: |
| \d | Any digit 0-9 | abc12\d | abc129 | abc12z |
| \D | Any non-digit | abc12\D | abc12z | abc123 |
| \s | Most whitespace (tab, space, newline, etc.) | hi\smum | hi mum | himum |
| + | One or more of the previous character | x+yz | xxxxxyz | yz |
| * | Zero or more of the previous character | x*yz | yz | abc |
| {n} | Previous character exactly n times | x{3}yz | xxxyz | xxyz |
| [xyz] | One of the characters in the list | [abc]99 | b99 | d99 |
| ^ | Start of string | ^abc | abc123 | 123abc |
| $ | End of string | abc$ | 123abc | abc123 |


### Notes

Normally the match can occur anywhere in the string.

Matching is usually case-sensitive.

To escape a special character, use backslash.
