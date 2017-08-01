using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using NUnit.Framework;
using System.Text.RegularExpressions;

namespace RegEx {
    [TestFixture]
    public class Tests {
        [Test]
        public void try_regex() {
            Assert.IsTrue(Regex.IsMatch("hi mum", ".*"));
            Assert.IsTrue(Regex.IsMatch("hi mum", "hi mum"));
            Assert.IsTrue(Regex.IsMatch("hi mum", ""));
        }

        [Test]
        public void matches_when_matches_exact_string() {
            Assert.IsTrue(Matcher.Matches("hi mum", "hi mum"));
        }

        [Test]
        public void matches_when_exact_string_exists_in_input() {
            Assert.IsTrue(Matcher.Matches("hi mum", "i m"));
        }

        [Test]
        public void dot_matches_any_character() {
            Assert.IsTrue(Matcher.Matches("hi mum", ".i.m.m"));
        }
    }
}
