#include <cctype>
#include <iostream>
#include <string>

using namespace std;

void get_sum(const string& data, const unsigned int offset)
{
	int sum = 0;
	const auto total = data.size();
	for (auto i = 0u; i < total; ++i) {
		if (data[i] == data[(i + offset) % total]) {
			sum += data[i] - '0';
		}
	}

	cout << "Sum: " << sum << endl;
}

int main()
{
	string data;
	cin >> data;

	get_sum(data, 1);
	get_sum(data, data.size() / 2);

	return 0;
}
