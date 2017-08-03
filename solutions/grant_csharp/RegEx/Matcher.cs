using System;
using System.Collections.Generic;
using System.Linq;

namespace RegEx
{
    public class Matcher
    {
        const int ASCII_ZERO = 48;
        const int ASCII_NINE = 57;

        private static Dictionary<String, Predicate<char>> tokenMatchers = new Dictionary<string, Predicate<char>> {
            {@".", inputChar => true },
            {@"\d", inputChar => inputChar >= ASCII_ZERO && inputChar <= ASCII_NINE },
            {@"\D", inputChar => inputChar > ASCII_NINE || inputChar < ASCII_ZERO }
        };

        public static bool Matches(string input, string pattern) {
            int cursor = 0;

            foreach(var token in GetTokens(pattern)) {

                var inputChar = input.Substring(cursor, 1).ElementAt(0);

                if(inputChar.ToString() == token) { // literal
                    cursor++;
                }
                else if (tokenMatchers.Keys.Contains(token) && 
                         tokenMatchers[token].Invoke(inputChar)) {
                        cursor++;
                }
                else {
                        return false;
                }
            }

            return true;
        }

        private static IEnumerable<string> GetTokens(string pattern) {
            string current = "";
            foreach (var character in pattern) {
                current += character;
                if (character != '\\') {
                    yield return current;
                    current = "";
                }
            }
        }
    }
}
