/**
 * This function helps organize the three vertices for G_TR1 and G_TR2
 * @param {int} v0 - first vertex index
 * @param {int} v1 - second vertex index
 * @param {int} v2 - third vertex index
 * @param {int} flag - order of vertices in command (where flag == primary vertex index)
 * @returns {int[]} - properly ordered vertex indices
**/
function triVertHelp(v0, v1, v2, flag) {
  switch (flag) {
    case 0:
      return [v0,v1,v2]
      break;
    case 1:
      return [v1,v2,v0]
      break;
    case 2:
      return [v2,v0,v1]
      break;
    default:
      console.log(`triVertHelp inproper flag value: ${flag}`)
      throw new Error("triVertHelp() improper flag input!")
  }
  return false
}

module.exports = triVertHelp
