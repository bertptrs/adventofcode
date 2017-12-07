using System;
using System.Collections.Generic;
using System.Linq;

public class Solution {

	static public void Main ()
	{
		Dictionary<string, string> parents = new Dictionary<string, string>();
		Dictionary<string, int> weights = new Dictionary<string, int>();
		Dictionary<string, HashSet<string>> children = new Dictionary<string, HashSet<string>>();

		string line = Console.ReadLine();
		do {
			line = line.Trim();
			string[] parts = line.Split(new string[] {" -> "}, StringSplitOptions.None);
			string[] nodeInfoParts = parts[0].Split(new char[] {' '});
			string parent = nodeInfoParts[0];
			int weight = int.Parse(nodeInfoParts[1].Substring(1, nodeInfoParts[1].Length - 2));

			weights.Add(parent, weight);
			children.Add(parent, new HashSet<string>());

			if (parts.Length == 2) {
				string[] curChildren = parts[1].Trim().Split(new string[] {", "}, StringSplitOptions.None);

				foreach (string child in curChildren) {
					parents.Add(child, parent);
					children[parent].Add(child);
				}
			}

			line = Console.ReadLine();
		} while (line != null);

		string root = RootNode(parents);
		Console.WriteLine(root);
		WrongWeight(root, children, weights);
	}

	static private string RootNode(Dictionary<string, string> parents) {
		string current = parents.Keys.First();

		while (parents.ContainsKey(current)) {
			current = parents[current];
		}

		return current;
	}

	static private void WrongWeight(string root, Dictionary<string, HashSet<string>> children, Dictionary<string, int> weights) {
		try {
			TowerWeight(root, children, weights);
		} catch (System.ArgumentException ex) {
			// Do nothing
		}
	}


	static private int TowerWeight(string current, Dictionary<string, HashSet<string>> children, Dictionary<string, int> weights) {
		if (children[current].Count == 0) {
			return weights[current];
		}

		int[] childWeights = new int[children[current].Count];
		string[] childNames = new string[children[current].Count];

		int i = 0;
		foreach (string child in children[current]) {
			int weight = TowerWeight(child, children, weights);
			childWeights[i] = weight;
			childNames[i] = child;
			i++;
		}

		int misMatches = 0;
		int misMatchIndex = -1;
		for (i = 1; i < children[current].Count; i++) {
			if (childWeights[i] != childWeights[0]) {
				misMatches++;
				misMatchIndex = i;
			}
		}

		int wrongIndex;
		int delta;
		switch (misMatches) {
			case 0:
				int finalWeight = children[current].Count * childWeights[0] + weights[current];
				// Tower is okay
				return finalWeight;

			case 1:
				wrongIndex = misMatchIndex;
				delta = childWeights[misMatchIndex] - childWeights[0];
				break;

			default:
				wrongIndex = 0;
				delta = childWeights[0] - childWeights[1];
				break;
		}

		Console.WriteLine(childNames[wrongIndex] + " " + (weights[childNames[wrongIndex]] - delta));

		throw new ArgumentException(current);
	}
}
