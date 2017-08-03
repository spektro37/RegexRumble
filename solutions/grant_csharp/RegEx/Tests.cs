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

            Assert.IsTrue(Regex.IsMatch("abc", "abc"), "abc");
            Assert.IsFalse(Regex.IsMatch("xyz", "abc"), "abc");

            Assert.IsTrue(Regex.IsMatch("axcye", "a.c.e"), "a.c.e");
            Assert.IsFalse(Regex.IsMatch("abbbcde", "a.c.e"), "a.c.e");

            Assert.IsTrue(Regex.IsMatch("abc129", @"abc12\d"), @"abc12\d");
            Assert.IsFalse(Regex.IsMatch("abc12z", @"abc12\d"), @"abc12\d");

            Assert.IsTrue(Regex.IsMatch("abc12z", @"abc12\D"), @"abc12\D");
            Assert.IsFalse(Regex.IsMatch("abc123", @"abc12\D"), @"abc12\D");

            Assert.IsTrue(Regex.IsMatch("abz", @"ab\w"), @"ab\w");
            Assert.IsFalse(Regex.IsMatch("ab:", @"ab\w"), @"ab\w");

            Assert.IsTrue(Regex.IsMatch("hi mum", @"hi\smum"), @"hi\smum");
            Assert.IsFalse(Regex.IsMatch("himum", @"hi\smum"), @"hi\smum");

            Assert.IsTrue(Regex.IsMatch("xxxxxyz", @"x+yz"), @"x+yz");
            Assert.IsFalse(Regex.IsMatch("yz", @"x+yz"), @"x+yz");

            Assert.IsTrue(Regex.IsMatch("yz", @"x*yz"), @"x*yz");
            Assert.IsFalse(Regex.IsMatch("abc", @"x*yz"), @"x*yz");

            Assert.IsTrue(Regex.IsMatch("xxxyz", @"x{3}yz"), @"x{3}yz");
            Assert.IsFalse(Regex.IsMatch("xxyz", @"x{3}yz"), @"x{3}yz");

            Assert.IsTrue(Regex.IsMatch("b99", @"[abc]99"), @"[abc]99");
            Assert.IsFalse(Regex.IsMatch("d99", @"[abc]99"), @"[abc]99");

            Assert.IsTrue(Regex.IsMatch("abc123", @"^abc"), @"^abc");
            Assert.IsFalse(Regex.IsMatch("123abc", @"^abc"), @"^abc");

            Assert.IsTrue(Regex.IsMatch("123abc", @"abc$"), @"abc$");
            Assert.IsFalse(Regex.IsMatch("abc123", @"abc$"), @"abc$");
        }

        [Test]
        public void matches_when_matches_exact_string() {
            Assert.IsTrue(Matcher.Matches("hi mum", "hi mum"));
        }

        [Test]
        public void dot_matches_any_character() {
            Assert.IsTrue(Matcher.Matches("hi mum", ".i.m.m"));
        }

        [Test]
        public void backslash_d_matches_any_digit() {
            Assert.IsTrue(Matcher.Matches("12345678", @"1\d3\d5\d7\d"));
            Assert.IsFalse(Matcher.Matches("12z", @"12\d"));
        }

        [Test]
        public void backslash_D_matches_any_non_digit() {
            Assert.IsTrue(Matcher.Matches("abc123x-!", @"abc123\D\D\D"));
            Assert.IsFalse(Matcher.Matches("123", @"\D\D\D"));
        }
    }
}
