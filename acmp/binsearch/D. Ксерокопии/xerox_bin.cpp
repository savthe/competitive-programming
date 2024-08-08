#include <iostream>
using namespace std;
int main()
{
	int64_t n, x, y;
	cin >> n >> x >> y;
	
	int low = 0, high = 2'000'000'000;
	while(low < high)
	{
		int m = (low + high)/2;
		if(m/x + m/y < n-1) low = m + 1;
		else high = m;
	}

	cout << low + min(x,y) << endl;
}

	/*
	while(b - a > 0)
	{
		int m = (a+b)/2;
		if(m/x+m/y >= n-1) b = m;
		else a = m+1;
	}
	*/

