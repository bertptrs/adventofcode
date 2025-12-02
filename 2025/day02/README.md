## Day 02: Terraform

The code assumes an input file at `../inputs/02.txt`. Other than that, simply try to run as follows:

```console
$ terraform init
…
Initializing the backend...
Initializing modules...
Initializing provider plugins...

Terraform has been successfully initialized!

You may now begin working with Terraform. Try running "terraform plan" to see
any changes that are required for your infrastructure. All Terraform commands
should now work.

If you ever set or change modules or backend configuration for Terraform,
rerun this command to reinitialize your working directory. If you forget, other
commands will detect it and remind you to do so if necessary.
$ terraform plan
….

Changes to Outputs:
  + part1 = secret
  + part2 = also secret

You can apply this plan to save these new output values to the Terraform state, without changing any real infrastructure.
```
