#include <iostream> 
using namespace std;
int main()
{
	uint64_t w, h, n;
	cin >> w >> h >> n;
	uint64_t a = 1, b = max(w*n, h*n);
	while(a < b)
	{
		uint64_t m = (a+b)/2;
		if((m/w)*(m/h) >= n) b = m;
		else a = m + 1;
	}
	cout << a << endl;
}

