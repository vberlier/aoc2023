print(sum((x:=[int(d)for d in l if d.isdigit()])[0]*10+x[-1]for l in open('day01.txt')))
print(sum((x:=[int(d)for(i,c)in enumerate(l)if(d:=c).isdigit()or(d:=next((j+1 for(j,n)in enumerate('one two three four five six seven eight nine'.split())if l.startswith(n,i)),0))])[0]*10+x[-1]for l in open('day01.txt')))
