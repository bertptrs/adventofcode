# Bonus Challenge

This part of the repo holds the bonus challenge for 2024: implement as much as possible in pure
Terraform.

## Usage

The infrastructure definitions expect the input files to be located at `../inputs/[\d]{2}.txt`.
After storing the input files there, the code should be runnable as follows:

```console
$ terraform init
< a lot of modules being installed >
$ terraform plan

Changes to Outputs:
  + day01_1 = 42
  + day01_2 = 12
  + day02_1 = …

You can apply this plan to save these new output values to the Terraform state, without changing any real infrastructure.
```

Note that Terraform may freeze for tens of seconds while it's running the computations. This is
normal, and all Terraform code ought to think a little before doing anything. It would save people
some bad rollbacks.

## Development

I use [Terraform tests](https://developer.hashicorp.com/terraform/language/tests) to automatically
run my terraform code on the sample inputs. It's almost a normal development workflow.

The only rule is that the code should be all terraform, no cheating by shelling out to external
programs. Using providers is allowed, as long as the providers don't actually interact with external
systems and are reasonably self-contained. I will try to limit my use of those regardless.

## Why

DevOps will continue until morale improves. But really, a friend remarked that my Python solutions
were strangely normal for me, so I opted to use a language that I do use professionally.

Terraform is a unique beast. It can do a lot, but it is also very limited, and intentionally so.
There's a standard library of functions you might want to use, but all of them work in strange ways
and there isn't that much to begin with. You can never mutate any variables, you can only declare
new ones. You don't have recursion and your only source of loops are list- and map comprehensions,
or multiple instantiations or a module.

These make for a very constrained programming environment, and constrained programming is fun. It
makes you think outside the box.
