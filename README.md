## Nuke Doctor

Tool that rewrites nuke scripts that take too long to open
or even crash. This tool probably does not fix the issue itself,
but helps identifying the node that causes the issue.
The tool does not touch the original file. It saves
a new one next to it with the name postfixed: ".doctored".

### usage example:
```
$ ./nukedoctor comp_scene_v001.nk --nocmd -l 7 --emptynodes --emptygroups -i Dot NoOp
```

the scene will be rewritten as:
`comp_scene_v001.nk.doctored`

The **json file is a data dump of the nodes** found in the nuke scene.

```
Usage: nukedoctor [OPTIONS] <script>

Arguments:
  <script>  

Options:
  -c, --nocmd              Don't write commands. (set, push, ...)
  -l <maxbodylines>        Only write nodes with less lines than <maxbodylines>.
  -e, --emptynodes         Writes nodes that are filtered out with the name only. Helps keeping the node tree while getting rid of heavy stuff.
  -g, --emptygroups        Writes Groups empty.
  -i <ignoretypes>...      Don't write nodes of these types. (Dot, NoOp, ...)
  -h, --help               Print help information
  -V, --version            Print version information
```

### Example scene sources:

[BenMcEwan](https://github.com/BenMcEwan/nuke_public)  
[chrisfryer](https://www.chrisfryer.co.uk/post/cf_tools-demo-nuke-script)

## building

#### [**Download**](https://rustup.rs/) [Rust](https://www.rust-lang.org/)

```
# build the binary:
NukeDoctor/ $> cargo build --release

# oprionally make the binary smaller:
NukeDoctor/ $> strip ./target/release/nukedoctor

# The biary is this file:
# target/release/nukedoctor
```
