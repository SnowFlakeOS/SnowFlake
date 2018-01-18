//=======================================================================
// Copyright Baptiste Wicht 2013-2016.
// Distributed under the terms of the MIT License.
// (See accompanying file LICENSE or copy at
//  http://www.opensource.org/licenses/MIT)
//=======================================================================

#ifndef TYPES_HPP
#define TYPES_HPP

#include <stddef.h>

typedef uint32_t	hashvalue_t;	///< Value type returned by the hash functions.
typedef size_t		streamsize;	///< Size of stream data
typedef size_t		uoff_t;		///< A type for storing offsets into blocks measured by size_t.
typedef uoff_t		streamoff;	///< Offset into a stream

enum class align_val_t : size_t {};

#endif
