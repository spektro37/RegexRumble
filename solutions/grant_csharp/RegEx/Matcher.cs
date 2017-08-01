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
            return input.Contains(pattern);
        }
    }
}
