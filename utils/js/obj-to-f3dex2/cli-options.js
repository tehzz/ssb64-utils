const SCRIPT_NAME = ( process.argv[ 1 ] || '' ).split( '/' ).pop()

const options = [{
  name: 'scale',
  short: 's',
  type: 'float',
  description: `Set a scaling factor for the input vertices`,
  example: `'${SCRIPT_NAME} --scale=2.0' or '${SCRIPT_NAME} -s 3.5'`
}]

module.exports = options
