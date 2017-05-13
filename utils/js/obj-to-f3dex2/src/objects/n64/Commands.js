/** Simple export of all written/necessary FEDEX2 commands
 * 0x01 G_VTX
 * 0x05 G_TRI1
 * 0x06 G_TRI2
 * 0x07 G_QUAD
 * 0xDF G_ENDDL
 * 0xE7 G_RDPPIPESYNC
**/

module.exports = {
  G_TRI1 : require('./cmds/TRI1.js'),
  G_TRI2 : require('./cmds/TRI2.js')
}
