/*
1）当num=0或1时，返回true。
2）设置左边界left为2，右边界right为num/2，取mid=(left+right)/2。
3）设置一个猜测值guess_squared，计算guess_squared=mid×mid并与num做比较。
当guess_squared=num时，说明num是一个完全平方数，返回true。
当guess_squared>num时，说明猜测的数大了，设置右边界right=mid-1。
当guess_squared<num时，说明猜测的数小了，设置左边界为left=mid+1。
4）重复步骤3，若直到left>right也没有找到，说明num不是完全平方数，返回false。
*/

pub mod solution1;
