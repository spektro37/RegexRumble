using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace RegEx
{
    public class Matcher
    {
        public static bool Matches(string input, string pattern) {
            int cursor = 0;

            foreach(var character in pattern) {
                if(input.ElementAt(cursor) == character || character == '.' ) {

                    cursor++;
                }
                else {
                        return false;
                }
            }

            return true;
        }
    }
}
