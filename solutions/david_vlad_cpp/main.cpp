#include <iostream>
#include <string>
#include <vector>

using namespace std;
int tokenise(string pattern, string tokens[]);

bool isMatch(char c, string token)
{
	if (token == ".")
		return true;

	if (token[0] == '\d')
		return isdigit(c);

	if (token[0] == '\w')
		return isalpha(c) || c == '_';

	if (token[0] == '\s')
		return c == '\t' || c == '\n' || c == '\r' || c == ' ';

	if (token[0] == '\D')
		return !isdigit(c);

	return (c== token[0]);
}

int findFirst(string input, string token, int startPosition)
{
	for(int i=startPosition; i < input.length(); i++)
	{
		if (isMatch(input[i],token)) return i;
	}
	return -1;
}

bool regEx(string input, string pattern)
{
	string tokens[pattern.length()];
	int numberOfTokens = tokenise(pattern, tokens);

	int start = findFirst(input,tokens[0],0);
	while (start != -1)
	{
		int patternCounter = 0;
		bool ok = true;
		for(int i = start; i < input.length() && patternCounter < numberOfTokens; i++)
		{
			if(isMatch(input[i],tokens[patternCounter]))
			{
				//cout << input[i] << " matches " << pattern[patternCounter] << endl;
				patternCounter++;
			}
			else
			{
				start = findFirst(input,tokens[0],start+1);
				ok=false;
				break;
			}
		}
		if (ok)
			return (patternCounter == pattern.length());
	}
	return false;
}
int tokenise(string pattern, string tokens[])
{
	int i = 0, tokensEntered = 0;
	while(i < pattern.length())
	{
		if(pattern[i] == '\\')
		{
			tokens[tokensEntered] = '\\' + pattern[i+1];
			i += 2;
			tokensEntered++;
		}
		else
		{
			tokens[tokensEntered] = pattern[i];
			i++;
			tokensEntered++;
		}
	}
	return tokensEntered;
}
void unitTestSuccess(string input, string pattern)
{
	cout << "Success Case: " << input << endl;
	if (regEx(input, pattern))
		cout << "OK" << endl;
	else
		cout << "Failed" << endl;
}

void unitTestFailure(string input, string pattern)
{
	cout << "Failure Case: " << input << endl;
	if (!regEx(input, pattern))
		cout << "OK" << endl;
	else
		cout << "Failed" << endl;
}

int main()
{
	string pattern = "abc";

	unitTestSuccess("ababce", pattern);
	unitTestSuccess("ababc", pattern);
	unitTestSuccess("aabc", pattern);
	unitTestSuccess("abc", pattern);
	unitTestFailure("xyz", pattern);
	cout << endl;

	pattern = "a.c.";
	unitTestSuccess("bababce", pattern);
	unitTestFailure("ababc", pattern);
	unitTestFailure("aabc", pattern);
	unitTestSuccess("abcd", pattern);
	unitTestFailure("xyzf", pattern);
	cout << endl;

	pattern = "_1b\w";
	unitTestSuccess("ba_1bd4", pattern);
	unitTestFailure("abbc", pattern);
	unitTestFailure("_ab.", pattern);
	unitTestSuccess("_1bw", pattern);
	unitTestFailure("xyzf", pattern);
	cout << endl;

	pattern = "\dbc\d";
	unitTestSuccess("bab1bc4", pattern);
	unitTestFailure("abbc", pattern);
	unitTestFailure("aabc", pattern);
	unitTestSuccess("9bc3", pattern);
	unitTestFailure("xyzf", pattern);
	cout << endl;

	pattern = "\D\wb_";
	unitTestSuccess("ba?ab_4", pattern);
	unitTestFailure("abbc", pattern);
	unitTestFailure("1ab_", pattern);
	unitTestSuccess("\\nGb_", pattern);
	unitTestFailure("d1b_", pattern);
	cout << endl;

	pattern = " _b\s";
	unitTestSuccess("b _b\n4", pattern);
	unitTestFailure(" bbc", pattern);
	unitTestFailure("aab\t", pattern);
	unitTestSuccess("\t1b\r", pattern);
	unitTestFailure("xyzf", pattern);
	cout << endl;
	return 0;
}
