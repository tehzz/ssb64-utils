const int = Object.create(null)

/**
 * Convert an input float into a fixed point int
 * @param {float} input - number to convert
 * @param {bool} s - number of sign bits
 * @param {int} m - number of integer bits
 * @param {int} f - number of fractional bit
 * @returns {int}
**/
int.toFixedPoint = function(s, m , f, input) {
  let bits = m + f + (s ? 1 : 0)
  let f_bits = 1 << f
  let upper = (1 << m) - (1 / f_bits)
  let lower = s ? -(1 << m) : 0

  // limit input to range
  if ( input > upper ) { input = upper }
  else if ( input < lower) { input = lower}

  return (input * f_bits) & ((1 << bits) - 1 )
}

int.toS8 = function( a ){
  let out = typeof a === 'number' ? a << 0 : parseInt(a,16)

  out &= 0xFF

  if ( out & 0x80 ) out -= 0x100

  return out
}

int.toU8 = function( a ){
  let out = typeof a === 'number' ? a << 0 : parseInt(a,16)

  return (out >>> 0) & 0xFF
}

// convert to fixed point signed 8bit s0.7
int.toFixedS8_7 = function ( float ) {
  // range: -1 to 0.9921875
  if ( float > 0.9921875 ) { float = 0.9921875 }
  else if (float < -1) { float = -1 }

  return (float * 128) & 0xFF
}

// convert to fixed point signed 16bit s10.5
int.toFixedS16_5 = function ( float ) {
  // range: -1024 to 1023.96875
  if (float > 1023.96875) { float = 1023.96875 }
  else if (float < -1024) { float = -1024 }

  return (float * 32) & 0xFFFF
}

module.exports = int
