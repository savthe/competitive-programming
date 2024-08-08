#include <iostream>
#include <vector>

using namespace std;

vector<int> v;

// проверяем, можем ли сделать не более k томов, 
// каждый из не более m страниц
bool test(int m, int k)
{
	for(int i = 0; i < v.size();)
	{
		int s = 0;
		while(i < v.size() && s + v[i] <= m)
			s += v[i++];
		if(s == 0) return false;
		--k;
	}
	return k >= 0;
}

int main()
{
	int n; cin >> n;
	v.resize(n);
	for(int i = 0; i < n; ++i) cin >> v[i];
	int k; cin >> k;

	int a = 0, b = 32767;
	while(a < b)
	{
		int m = (a+b)/2;
		if(test(m, k)) b = m;
		else a = m + 1;
	}
	cout << a << endl;
}



