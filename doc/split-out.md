# `mgt split-out --help`

```
create a new branch with this repository's history rewritten according to the repo file rules

USAGE:
    mgt split-out [FLAGS] [OPTIONS] <repo_file>

FLAGS:
    --verbose              Prints verbose information 
    --dry-run              Print out the steps taken, but don't actually run or change anything. 
    -h, --help             

OPTIONS:
    -o, --output-branch OUTPUT-BRANCH    name of branch that will be created with new split 
                                         history 
    -r, --rebase REBASE                  after generating a branch with rewritten history, rebase 
                                         that branch such that it can be fast forwarded back into 
                                         the comparison branch. for split-in that is the branch 
                                         you started on. For split-out, that is the remote branch. 
                                         Optionally provide a '--rebase BRANCH-NAME' to rebase 
                                         onto that branch instead of the default. 
    -t, --topbase TOPBASE                like --rebase, but it finds a fork point by stopping at 
                                         the first commit that two branches have in common. This 
                                         is useful as an 'update' mechanism. Optionally provide a 
                                         '--topbase BRANCH-NAME' to topbase onto that branch 
                                         instead of the default. 

POSITIONAL:
    <repo-file>    path to file that contains instructions of how to split a repository
```
