const SCRIPT_NAME = ( process.argv[ 1 ] || '' ).split( '/' ).pop()

const options = [{
  name: 'file',
  short: 'f',
  type: 'path',
  description: 'The input ".obj" file to be converted',
  example: `'${SCRIPT_NAME} --file=foo.obj' or '${SCRIPT_NAME} -f bar.obj'`,
},
{
  name: 'scale',
  short: 's',
  type: 'float',
  description: `Set a scaling factor for the input vertices`,
  example: `'${SCRIPT_NAME} --scale=2.0' or '${SCRIPT_NAME} -s 3.5'`
},
{
  name: 'output',
  short: 'o',
  type: 'path',
  description: 'The name of the output display list file',
  example: `'${SCRIPT_NAME} --output=mario.c' or '${SCRIPT_NAME} -o luigi.asm'`
},
{
  name: 'type',
  short: 't',
  type: 'string',
  description:  `Set the output file type.
                There are two options: 'c' and 'bass'`,
  example: `'${SCRIPT_NAME} --type=c' or '${SCRIPT_NAME} -t bass'`
}]

module.exports = options
