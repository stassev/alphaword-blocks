# AlphaWord Blocks — Julia spellability verifier for a candidate block set
# Copyright (C) 2025- Svetlin Tassev
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# === Imports (preserved) ===
using Unicode
using Random
using Base.Threads

# === Fixed cubes you provided (no learning/building, only checking) ===
#const cubes = [
#    Set(['e', 'j', 'm', 'o', 'u', 'w']),
#    Set(['c', 'n', 'o', 'q', 's', 't']),
#    Set(['a', 'c', 'l', 's', 'w', 'z']),
#    Set(['b', 'k', 'n', 'o', 'q', 'u']),
#    Set(['d', 'f', 'h', 'i', 's', 'u']),
#    Set(['b', 'c', 'i', 'n', 'u', 'v']),
#    Set(['f', 'g', 'i', 'l', 's', 'u']),
#    Set(['a', 'c', 'e', 'h', 'l', 'v']),
#    Set(['r', 't', 'u', 'w', 'x', 'z']),
#    Set(['d', 'm', 'o', 's', 't', 'z']),
#    Set(['h', 'l', 'o', 's', 'u', 'x']),
#    Set(['a', 'c', 'i', 'r', 'y', 'z']),
#    Set(['d', 'l', 'r', 't', 'u', 'v']),
#    Set(['e', 'i', 'm', 'n', 'p', 'q']),
#    Set(['a', 'e', 'h', 'p', 'w', 'y']),
#    Set(['a', 'e', 'f', 'g', 'm', 'z']),
#    Set(['b', 'g', 'n', 'o', 'p', 'y']),
#    Set(['a', 'b', 'j', 'o', 's', 'y']),
#    Set(['c', 'd', 'k', 'p', 't', 'z']),
#    Set(['b', 'c', 'k', 'p', 's', 't']),
#    Set(['b', 'h', 'j', 'n', 't', 'v']),
#    Set(['e', 'g', 'i', 'k', 'l', 't']),
#    Set(['e', 'f', 'j', 'r', 'y', 'w']),
#    Set(['a', 'c', 'g', 'o', 'u', 'x']),
#    Set(['e', 'i', 'm', 'q', 'r', 'z']),
#    Set(['a', 'd', 'l', 'r', 'z', 'e'])
#]

#const cubes :: Vector{Set{Char}} = [Set{Char}(['i','j','o','t','u','z']), Set{Char}(['d','g','i','t','u','w']), Set{Char}(['o','t','v','w','y','z']), Set{Char}(['a','d','o','s','t','z']), Set{Char}(['l','r','s','w','x','z']), Set{Char}(['e','g','l','s','v','z']), Set{Char}(['f','i','l','m','o','z']), Set{Char}(['e','k','l','o','u','z']), Set{Char}(['p','r','s','t','u','z']), Set{Char}(['a','e','l','p','r','t']), Set{Char}(['h','k','r','s','u','y']), Set{Char}(['h','j','n','q','r','u']), Set{Char}(['e','o','s','u','v','x']), Set{Char}(['g','q','r','s','t','x']), Set{Char}(['a','b','d','h','m','o']), Set{Char}(['d','e','f','j','q','u']), Set{Char}(['b','c','i','m','n','y']), Set{Char}(['d','n','p','q','u','v']), Set{Char}(['c','e','f','l','m','n']), Set{Char}(['f','g','h','m','p','r']), Set{Char}(['a','c','e','i','j','o']), Set{Char}(['a','b','c','h','n','p']), Set{Char}(['c','d','f','k','n','y']), Set{Char}(['a','b','c','i','w','y']), Set{Char}(['a','b','c','e','g','m']), Set{Char}(['b','c','i','k','m','s'])]
#const cubes :: Vector{Set{Char}} = [Set{Char}(['b','n','o','u','w','z']), Set{Char}(['n','o','s','t','u','w']), Set{Char}(['e','o','s','t','u','z']), Set{Char}(['g','i','j','o','s','u']), Set{Char}(['e','f','j','k','l','o']), Set{Char}(['d','l','q','r','s','t']), Set{Char}(['e','n','p','q','t','v']), Set{Char}(['a','h','j','m','n','y']), Set{Char}(['h','i','p','r','y','z']), Set{Char}(['a','c','k','n','q','r']), Set{Char}(['b','k','l','m','t','z']), Set{Char}(['g','i','r','u','y','z']), Set{Char}(['a','d','r','v','y','z']), Set{Char}(['a','i','k','n','v','y']), Set{Char}(['a','g','r','s','w','x']), Set{Char}(['l','m','p','s','w','z']), Set{Char}(['f','g','h','i','m','q']), Set{Char}(['b','c','d','h','n','v']), Set{Char}(['e','f','p','s','u','x']), Set{Char}(['a','b','c','e','i','p']), Set{Char}(['c','d','i','m','t','u']), Set{Char}(['a','g','k','o','s','t']), Set{Char}(['c','e','j','l','o','z']), Set{Char}(['a','b','c','d','e','o']), Set{Char}(['b','c','d','e','h','u']), Set{Char}(['c','f','h','l','u','x'])]
#seed=2
#const cubes :: Vector{Set{Char}} = [Set{Char}(['c','p','s','t','v','z']), Set{Char}(['a','e','g','t','u','v']), Set{Char}(['j','n','o','u','v','w']), Set{Char}(['a','i','k','o','s','z']), Set{Char}(['e','r','s','u','y','z']), Set{Char}(['i','o','s','u','y','z']), Set{Char}(['h','o','s','u','w','z']), Set{Char}(['k','l','p','q','t','u']), Set{Char}(['d','l','q','r','u','z']), Set{Char}(['n','o','q','s','t','y']), Set{Char}(['e','h','l','p','s','w']), Set{Char}(['l','n','q','r','x','z']), Set{Char}(['m','p','t','u','v','y']), Set{Char}(['b','c','i','k','s','u']), Set{Char}(['a','d','f','m','r','z']), Set{Char}(['a','b','h','i','l','n']), Set{Char}(['a','b','c','e','n','x']), Set{Char}(['b','c','g','i','t','y']), Set{Char}(['a','b','f','g','i','j']), Set{Char}(['d','e','f','i','j','x']), Set{Char}(['a','b','d','m','o','p']), Set{Char}(['a','b','c','e','o','w']), Set{Char}(['c','d','e','g','m','r']), Set{Char}(['a','e','f','m','o','t']), Set{Char}(['a','g','h','k','l','n']), Set{Char}(['b','c','e','h','j','m'])]
#const cubes :: Vector{Set{Char}} = [Set{Char}(['h','o','u','v','y','z']), Set{Char}(['e','o','u','v','y','z']), Set{Char}(['l','o','r','u','v','z']), Set{Char}(['m','o','r','s','w','z']), Set{Char}(['g','l','o','s','t','z']), Set{Char}(['g','i','j','o','t','y']), Set{Char}(['f','i','n','o','s','u']), Set{Char}(['e','q','r','s','u','v']), Set{Char}(['d','g','i','r','w','z']), Set{Char}(['i','o','q','s','u','z']), Set{Char}(['e','j','l','t','w','x']), Set{Char}(['m','n','q','s','t','y']), Set{Char}(['h','n','p','u','w','x']), Set{Char}(['n','q','s','t','u','x']), Set{Char}(['b','e','m','n','p','z']), Set{Char}(['a','c','h','k','p','s']), Set{Char}(['a','c','k','m','p','t']), Set{Char}(['b','c','d','g','n','u']), Set{Char}(['b','c','d','e','g','k']), Set{Char}(['a','d','e','f','i','j']), Set{Char}(['a','b','d','e','h','l']), Set{Char}(['a','b','h','j','l','r']), Set{Char}(['a','c','i','k','l','t']), Set{Char}(['a','b','k','m','p','y']), Set{Char}(['a','c','d','f','h','i']), Set{Char}(['a','b','c','d','e','f'])]
#const cubes :: Vector{Set{Char}} = [Set{Char}(['e','j','p','s','t','u']), Set{Char}(['c','d','o','u','y','z']), Set{Char}(['c','f','g','o','q','t']), Set{Char}(['g','j','n','o','s','u']), Set{Char}(['e','s','u','v','w','y']), Set{Char}(['d','j','l','s','u','z']), Set{Char}(['e','k','l','m','q','z']), Set{Char}(['d','k','n','t','u','y']), Set{Char}(['h','n','q','r','u','z']), Set{Char}(['e','i','m','o','v','z']), Set{Char}(['b','c','m','n','s','z']), Set{Char}(['a','h','m','r','v','w']), Set{Char}(['a','e','m','q','s','x']), Set{Char}(['g','n','o','p','r','t']), Set{Char}(['a','b','c','l','w','y']), Set{Char}(['a','d','i','l','m','p']), Set{Char}(['c','f','i','u','x','z']), Set{Char}(['b','i','l','o','r','y']), Set{Char}(['b','h','k','l','r','x']), Set{Char}(['c','f','g','p','r','v']), Set{Char}(['a','b','i','n','t','u']), Set{Char}(['a','e','g','p','s','w']), Set{Char}(['b','d','e','i','o','t']), Set{Char}(['h','i','j','o','s','z']), Set{Char}(['a','b','d','e','f','t']), Set{Char}(['a','c','e','h','i','k'])]
#seed=5 skipped
#seed=6
#const cubes :: Vector{Set{Char}} = [Set{Char}(['g','i','o','u','y','z']), Set{Char}(['d','l','o','t','w','z']), Set{Char}(['h','m','s','t','w','z']), Set{Char}(['n','r','s','t','w','z']), Set{Char}(['i','s','t','u','y','z']), Set{Char}(['d','l','s','t','u','v']), Set{Char}(['i','j','o','q','r','u']), Set{Char}(['i','l','m','s','t','w']), Set{Char}(['b','c','e','n','o','u']), Set{Char}(['a','e','s','t','u','x']), Set{Char}(['c','f','l','p','s','u']), Set{Char}(['a','f','k','l','n','q']), Set{Char}(['g','h','i','p','q','u']), Set{Char}(['a','c','h','r','y','z']), Set{Char}(['a','g','n','o','u','y']), Set{Char}(['a','p','r','s','v','z']), Set{Char}(['c','e','m','n','q','v']), Set{Char}(['a','b','e','j','k','o']), Set{Char}(['a','d','e','h','m','z']), Set{Char}(['a','e','f','j','l','x']), Set{Char}(['c','f','m','o','p','v']), Set{Char}(['a','b','c','g','h','i']), Set{Char}(['b','e','j','k','n','y']), Set{Char}(['a','b','c','d','e','o']), Set{Char}(['d','g','k','p','r','x']), Set{Char}(['a','b','c','e','h','i'])]
#seed=7
#const cubes :: Vector{Set{Char}} = [Set{Char}(['f','j','o','s','y','z']), Set{Char}(['g','i','m','p','s','t']), Set{Char}(['h','o','t','u','v','z']), Set{Char}(['h','m','s','t','u','z']), Set{Char}(['e','g','n','o','t','u']), Set{Char}(['l','o','u','v','w','z']), Set{Char}(['l','o','q','t','u','z']), Set{Char}(['e','o','p','s','t','w']), Set{Char}(['h','o','q','s','u','w']), Set{Char}(['d','j','n','s','u','v']), Set{Char}(['a','h','p','r','v','z']), Set{Char}(['l','n','p','q','t','w']), Set{Char}(['c','e','u','x','y','z']), Set{Char}(['d','q','r','s','u','y']), Set{Char}(['a','b','j','m','n','y']), Set{Char}(['b','e','g','m','r','z']), Set{Char}(['a','b','c','i','k','r']), Set{Char}(['a','c','e','i','k','y']), Set{Char}(['g','k','m','p','r','x']), Set{Char}(['a','b','k','l','m','n']), Set{Char}(['a','c','d','f','h','i']), Set{Char}(['a','c','e','j','l','x']), Set{Char}(['c','d','i','j','l','s']), Set{Char}(['a','b','d','f','i','n']), Set{Char}(['a','c','d','e','i','o']), Set{Char}(['a','b','c','e','f','g'])]
#const cubes :: Vector{Set{Char}} = [Set{Char}(['j','l','o','s','v','x']), Set{Char}(['f','g','o','q','t','u']), Set{Char}(['i','j','l','o','t','u']), Set{Char}(['i','k','o','p','q','t']), Set{Char}(['h','i','s','w','y','z']), Set{Char}(['e','h','i','m','s','u']), Set{Char}(['h','i','o','q','r','w']), Set{Char}(['m','p','r','x','y','z']), Set{Char}(['a','c','g','n','y','z']), Set{Char}(['d','e','r','u','v','z']), Set{Char}(['b','g','j','n','r','v']), Set{Char}(['a','b','c','d','s','z']), Set{Char}(['a','f','l','s','u','z']), Set{Char}(['f','h','m','n','p','z']), Set{Char}(['a','k','n','q','t','x']), Set{Char}(['b','e','g','m','w','y']), Set{Char}(['e','k','s','u','v','z']), Set{Char}(['a','c','k','m','n','t']), Set{Char}(['a','b','i','j','o','w']), Set{Char}(['a','e','i','p','u','y']), Set{Char}(['a','b','c','l','n','s']), Set{Char}(['a','b','c','e','l','t']), Set{Char}(['a','c','e','o','p','t']), Set{Char}(['a','c','d','f','h','u']), Set{Char}(['b','c','d','e','g','l']), Set{Char}(['d','k','o','r','s','u'])]
#const cubes :: Vector{Set{Char}} = [Set{Char}(['a','l','m','r','v','z']), Set{Char}(['i','j','o','t','w','z']), Set{Char}(['e','j','o','s','u','z']), Set{Char}(['g','o','q','s','u','y']), Set{Char}(['l','n','o','u','w','z']), Set{Char}(['l','m','n','o','q','t']), Set{Char}(['d','i','k','l','q','u']), Set{Char}(['k','l','o','s','t','z']), Set{Char}(['l','n','p','s','u','y']), Set{Char}(['f','h','i','j','m','s']), Set{Char}(['e','g','j','n','v','x']), Set{Char}(['e','h','q','s','y','z']), Set{Char}(['i','p','r','u','v','z']), Set{Char}(['h','k','r','u','v','y']), Set{Char}(['a','c','d','e','s','x']), Set{Char}(['a','b','c','e','i','u']), Set{Char}(['e','q','s','t','x','z']), Set{Char}(['b','h','k','n','p','r']), Set{Char}(['b','f','h','i','r','w']), Set{Char}(['d','g','h','i','o','t']), Set{Char}(['a','c','e','p','t','x']), Set{Char}(['b','c','d','n','o','r']), Set{Char}(['a','b','c','e','f','j']), Set{Char}(['a','m','t','u','x','y']), Set{Char}(['c','d','f','g','m','p']), Set{Char}(['a','b','c','g','m','w'])]
#seed=10
#const cubes :: Vector{Set{Char}} = [Set{Char}(['f','o','t','u','v','z']), Set{Char}(['i','o','s','t','w','z']), Set{Char}(['b','l','t','u','w','z']), Set{Char}(['e','g','o','s','t','z']), Set{Char}(['a','i','s','u','x','z']), Set{Char}(['k','l','m','o','t','u']), Set{Char}(['i','j','q','r','u','v']), Set{Char}(['e','g','j','o','q','y']), Set{Char}(['g','k','n','o','r','y']), Set{Char}(['h','l','n','u','w','y']), Set{Char}(['e','l','n','p','q','u']), Set{Char}(['m','n','p','s','x','z']), Set{Char}(['g','n','p','s','t','z']), Set{Char}(['h','l','q','r','s','u']), Set{Char}(['h','s','t','u','v','w']), Set{Char}(['j','m','r','x','y','z']), Set{Char}(['a','d','h','i','v','y']), Set{Char}(['a','b','c','e','o','p']), Set{Char}(['c','d','e','i','j','o']), Set{Char}(['a','b','h','i','k','n']), Set{Char}(['a','c','d','e','f','m']), Set{Char}(['a','c','d','e','f','l']), Set{Char}(['b','c','d','e','f','k']), Set{Char}(['a','b','d','e','i','m']), Set{Char}(['a','c','p','q','s','x']), Set{Char}(['a','b','c','d','g','r'])]
#const cubes :: Vector{Set{Char}} = [Set{Char}(['e','o','s','u','v','z']), Set{Char}(['g','h','j','o','s','u']), Set{Char}(['g','i','j','o','t','z']), Set{Char}(['n','s','t','u','y','z']), Set{Char}(['o','q','s','t','u','z']), Set{Char}(['i','j','n','s','t','y']), Set{Char}(['a','h','q','r','w','z']), Set{Char}(['e','q','s','t','u','z']), Set{Char}(['m','o','p','s','y','z']), Set{Char}(['j','k','l','m','s','t']), Set{Char}(['a','g','i','m','n','w']), Set{Char}(['a','c','m','o','p','u']), Set{Char}(['f','h','l','n','q','y']), Set{Char}(['l','r','u','w','x','z']), Set{Char}(['b','i','u','w','x','y']), Set{Char}(['b','e','h','p','u','v']), Set{Char}(['a','b','d','r','t','x']), Set{Char}(['a','b','i','k','l','o']), Set{Char}(['c','d','e','g','p','r']), Set{Char}(['a','c','d','e','f','i']), Set{Char}(['a','c','k','n','r','v']), Set{Char}(['a','l','o','p','r','v']), Set{Char}(['a','e','g','h','l','m']), Set{Char}(['b','c','d','e','h','n']), Set{Char}(['c','d','e','f','k','m']), Set{Char}(['a','b','c','d','f','i'])]
#const cubes :: Vector{Set{Char}} = [Set{Char}(['b','e','o','u','w','z']), Set{Char}(['b','l','r','s','t','z']), Set{Char}(['a','i','j','r','s','z']), Set{Char}(['f','h','l','t','u','z']), Set{Char}(['f','n','o','t','u','w']), Set{Char}(['c','l','o','s','u','v']), Set{Char}(['i','n','o','q','u','v']), Set{Char}(['i','j','o','s','t','z']), Set{Char}(['g','i','m','t','u','w']), Set{Char}(['j','n','o','q','s','y']), Set{Char}(['d','m','p','s','y','z']), Set{Char}(['a','s','t','u','w','y']), Set{Char}(['i','p','t','u','x','z']), Set{Char}(['d','h','l','m','s','v']), Set{Char}(['a','e','h','n','v','y']), Set{Char}(['a','h','k','o','x','y']), Set{Char}(['d','e','h','k','r','x']), Set{Char}(['b','c','e','m','n','p']), Set{Char}(['c','d','i','k','l','x']), Set{Char}(['c','e','f','p','r','y']), Set{Char}(['b','c','e','i','j','o']), Set{Char}(['g','h','k','l','m','p']), Set{Char}(['a','b','c','d','e','g']), Set{Char}(['a','c','e','f','g','r']), Set{Char}(['b','d','n','q','r','u']), Set{Char}(['a','e','g','m','q','z'])]
#const cubes :: Vector{Set{Char}} = [Set{Char}(['i','o','s','t','u','x']), Set{Char}(['e','f','l','n','s','t']), Set{Char}(['b','e','h','i','s','t']), Set{Char}(['b','e','f','j','n','q']), Set{Char}(['b','e','j','l','n','p']), Set{Char}(['a','n','o','s','v','z']), Set{Char}(['a','c','h','i','r','u']), Set{Char}(['e','h','j','u','y','z']), Set{Char}(['h','i','p','r','u','z']), Set{Char}(['a','c','j','k','n','y']), Set{Char}(['h','m','p','r','x','z']), Set{Char}(['m','t','w','x','y','z']), Set{Char}(['d','r','s','u','w','y']), Set{Char}(['l','q','r','s','t','v']), Set{Char}(['c','f','m','p','q','t']), Set{Char}(['d','g','k','o','v','y']), Set{Char}(['b','g','k','r','s','y']), Set{Char}(['a','d','e','o','p','u']), Set{Char}(['c','f','h','i','k','o']), Set{Char}(['c','d','e','g','l','w']), Set{Char}(['a','g','i','t','v','z']), Set{Char}(['a','d','i','l','u','w']), Set{Char}(['b','m','n','o','q','s']), Set{Char}(['b','c','g','m','o','z']), Set{Char}(['a','d','e','h','m','u']), Set{Char}(['a','c','l','o','u','z'])]
#const cubes :: Vector{Set{Char}} = [Set{Char}(['f','i','o','p','q','s']), Set{Char}(['e','f','g','i','m','v']), Set{Char}(['c','e','g','k','q','z']), Set{Char}(['c','i','j','r','y','z']), Set{Char}(['r','s','t','u','y','z']), Set{Char}(['b','l','m','t','x','z']), Set{Char}(['h','j','n','p','q','w']), Set{Char}(['d','h','l','n','u','w']), Set{Char}(['c','k','l','n','r','w']), Set{Char}(['b','c','d','f','i','k']), Set{Char}(['a','n','p','x','y','z']), Set{Char}(['b','o','q','u','w','y']), Set{Char}(['g','m','p','t','u','x']), Set{Char}(['h','m','o','p','s','v']), Set{Char}(['b','e','f','g','l','m']), Set{Char}(['a','c','l','m','s','z']), Set{Char}(['c','f','s','t','w','x']), Set{Char}(['a','c','g','t','v','z']), Set{Char}(['e','i','l','s','t','y']), Set{Char}(['a','d','e','j','o','u']), Set{Char}(['d','h','k','o','r','u']), Set{Char}(['d','e','i','j','o','v']), Set{Char}(['a','b','d','e','o','s']), Set{Char}(['a','n','o','t','u','z']), Set{Char}(['e','h','n','r','s','u']), Set{Char}(['a','b','d','i','r','u'])]
#const cubes :: Vector{Set{Char}} = [Set{Char}(['a','k','r','v','y','z']), Set{Char}(['e','t','v','w','x','z']), Set{Char}(['e','f','o','u','w','z']), Set{Char}(['f','i','j','m','p','z']), Set{Char}(['a','l','m','o','v','z']), Set{Char}(['h','j','l','r','u','z']), Set{Char}(['j','n','o','q','y','z']), Set{Char}(['f','g','i','o','q','r']), Set{Char}(['g','n','o','s','u','w']), Set{Char}(['l','p','q','t','u','y']), Set{Char}(['e','r','s','t','u','y']), Set{Char}(['a','h','i','s','t','y']), Set{Char}(['i','p','s','t','v','x']), Set{Char}(['a','e','n','p','s','u']), Set{Char}(['d','g','m','o','t','u']), Set{Char}(['f','l','m','n','q','s']), Set{Char}(['a','i','o','r','t','u']), Set{Char}(['d','g','h','k','p','u']), Set{Char}(['b','c','e','k','w','z']), Set{Char}(['a','b','c','m','s','w']), Set{Char}(['a','b','c','d','h','t']), Set{Char}(['b','l','n','r','s','x']), Set{Char}(['c','d','e','i','j','o']), Set{Char}(['b','c','d','h','k','n']), Set{Char}(['a','c','e','f','l','m']), Set{Char}(['b','c','d','e','g','i'])]
#const cubes :: Vector{Set{Char}} = [Set{Char}(['e','l','r','u','w','z']), Set{Char}(['o','r','t','u','w','z']), Set{Char}(['b','d','g','s','t','u']), Set{Char}(['d','i','l','o','t','y']), Set{Char}(['d','l','n','r','s','t']), Set{Char}(['c','q','r','s','u','v']), Set{Char}(['e','k','o','q','r','w']), Set{Char}(['b','f','j','l','o','u']), Set{Char}(['k','o','p','r','s','t']), Set{Char}(['i','n','p','s','u','w']), Set{Char}(['h','n','p','q','v','z']), Set{Char}(['a','m','n','q','t','y']), Set{Char}(['e','l','m','s','v','y']), Set{Char}(['g','h','i','t','u','v']), Set{Char}(['b','c','i','n','u','z']), Set{Char}(['a','g','h','m','s','x']), Set{Char}(['a','f','m','p','x','z']), Set{Char}(['k','m','p','u','y','z']), Set{Char}(['d','f','h','j','o','z']), Set{Char}(['e','h','i','o','t','w']), Set{Char}(['a','b','c','i','o','y']), Set{Char}(['a','c','g','h','l','z']), Set{Char}(['a','b','c','e','j','s']), Set{Char}(['b','c','d','e','f','m']), Set{Char}(['c','e','j','k','n','x']), Set{Char}(['a','d','e','f','g','i'])]

#NEW:
#const cubes :: Vector{Set{Char}} = [Set{Char}(['l','p','r','t','u','z']), Set{Char}(['d','m','p','t','w','z']), Set{Char}(['e','q','r','t','u','z']), Set{Char}(['f','i','p','t','u','z']), Set{Char}(['d','l','n','q','s','w']), Set{Char}(['a','j','n','o','s','u']), Set{Char}(['e','j','n','p','r','x']), Set{Char}(['f','m','o','q','v','y']), Set{Char}(['g','h','s','u','y','z']), Set{Char}(['e','u','w','x','y','z']), Set{Char}(['g','h','s','t','u','z']), Set{Char}(['b','i','o','r','u','v']), Set{Char}(['l','n','o','q','s','t']), Set{Char}(['a','e','i','v','w','y']), Set{Char}(['f','k','m','r','s','w']), Set{Char}(['e','m','o','s','u','v']), Set{Char}(['a','c','k','o','s','y']), Set{Char}(['c','e','f','k','l','t']), Set{Char}(['a','b','c','o','r','z']), Set{Char}(['b','d','g','j','l','m']), Set{Char}(['a','c','d','i','o','p']), Set{Char}(['a','b','h','l','m','x']), Set{Char}(['b','c','d','e','i','n']), Set{Char}(['b','c','e','g','h','i']), Set{Char}(['a','e','g','i','m','n']), Set{Char}(['c','d','e','h','j','k'])]
#const cubes :: Vector{Set{Char}} = [Set{Char}(['i','q','r','t','w','y']), Set{Char}(['e','o','q','t','u','z']), Set{Char}(['m','o','r','t','u','z']), Set{Char}(['e','g','o','s','v','y']), Set{Char}(['l','n','o','s','t','z']), Set{Char}(['g','i','k','l','t','z']), Set{Char}(['d','n','p','q','r','v']), Set{Char}(['j','k','l','o','s','u']), Set{Char}(['e','g','l','n','p','u']), Set{Char}(['f','k','n','p','q','t']), Set{Char}(['f','i','n','o','s','v']), Set{Char}(['d','e','s','u','w','z']), Set{Char}(['a','b','c','l','s','u']), Set{Char}(['g','h','t','u','x','y']), Set{Char}(['a','c','u','w','y','z']), Set{Char}(['h','i','u','w','y','z']), Set{Char}(['c','e','j','p','w','z']), Set{Char}(['c','e','h','i','m','s']), Set{Char}(['a','f','m','s','v','x']), Set{Char}(['a','b','c','d','e','j']), Set{Char}(['c','e','h','n','o','p']), Set{Char}(['a','d','i','j','o','x']), Set{Char}(['a','b','c','e','g','m']), Set{Char}(['b','d','e','h','k','r']), Set{Char}(['a','b','d','e','l','r']), Set{Char}(['a','b','c','f','i','m'])]
#const cubes :: Vector{Set{Char}} = [Set{Char}(['l','t','u','w','x','z']), Set{Char}(['o','r','s','t','y','z']), Set{Char}(['l','o','r','t','w','z']), Set{Char}(['a','i','o','s','v','z']), Set{Char}(['i','j','o','r','w','z']), Set{Char}(['b','e','m','n','t','z']), Set{Char}(['e','o','s','u','y','z']), Set{Char}(['j','k','l','n','q','s']), Set{Char}(['j','m','n','o','q','s']), Set{Char}(['e','g','p','s','t','u']), Set{Char}(['a','i','l','m','s','u']), Set{Char}(['g','h','i','u','v','x']), Set{Char}(['a','d','h','n','t','u']), Set{Char}(['i','k','t','x','y','z']), Set{Char}(['b','c','e','f','s','u']), Set{Char}(['f','h','i','p','w','y']), Set{Char}(['d','f','p','r','u','v']), Set{Char}(['a','b','c','d','l','m']), Set{Char}(['c','e','h','j','l','u']), Set{Char}(['c','e','m','n','r','v']), Set{Char}(['a','d','g','i','p','w']), Set{Char}(['a','b','c','d','e','f']), Set{Char}(['e','f','k','p','q','r']), Set{Char}(['a','b','c','g','n','y']), Set{Char}(['c','f','g','h','o','q']), Set{Char}(['a','b','c','e','k','o'])]
const cubes :: Vector{Set{Char}} = [Set{Char}(['l','t','u','w','x','z']), Set{Char}(['d','o','r','t','y','z']), Set{Char}(['l','o','r','t','w','z']), Set{Char}(['a','i','o','s','v','z']), Set{Char}(['i','j','o','r','w','z']), Set{Char}(['b','e','m','n','t','z']), Set{Char}(['e','o','s','u','y','z']), Set{Char}(['j','k','l','n','q','s']), Set{Char}(['j','m','n','o','q','s']), Set{Char}(['e','g','p','s','t','u']), Set{Char}(['a','i','l','m','s','u']), Set{Char}(['h','i','s','u','v','x']), Set{Char}(['a','g','h','n','t','u']), Set{Char}(['i','k','t','x','y','z']), Set{Char}(['b','c','e','f','s','u']), Set{Char}(['f','h','i','p','w','y']), Set{Char}(['d','f','p','r','u','v']), Set{Char}(['a','b','c','d','l','m']), Set{Char}(['c','e','h','j','l','u']), Set{Char}(['c','e','m','n','r','v']), Set{Char}(['a','d','g','i','p','w']), Set{Char}(['a','b','c','d','e','f']), Set{Char}(['e','f','k','p','q','r']), Set{Char}(['a','b','c','g','n','y']), Set{Char}(['c','f','g','h','o','q']), Set{Char}(['a','b','c','e','k','o'])]




# === Transliteration map (preserved subset; comments kept) ===
const translit_map = Dict{Char,String}()

# Western diacritics to ASCII (matches your JL)
translit_map['á'] = "a"; translit_map['à'] = "a"; translit_map['ä'] = "ae"; translit_map['â'] = "a"
translit_map['ã'] = "a"; translit_map['å'] = "a"; translit_map['ā'] = "a"; translit_map['ą'] = "a"
translit_map['æ'] = "ae"; translit_map['œ'] = "oe"; translit_map['ø'] = "o"
translit_map['ç'] = "c"; translit_map['č'] = "c"; translit_map['ć'] = "c"; translit_map['ċ'] = "c"
translit_map['ď'] = "d"; translit_map['đ'] = "d"
translit_map['é'] = "e"; translit_map['è'] = "e"; translit_map['ë'] = "e"; translit_map['ê'] = "e"
translit_map['ē'] = "e"; translit_map['ė'] = "e"; translit_map['ę'] = "e"
translit_map['ğ'] = "g"; translit_map['ģ'] = "g"; translit_map['ĝ'] = "g"
translit_map['í'] = "i"; translit_map['ì'] = "i"; translit_map['ï'] = "i"; translit_map['î'] = "i"
translit_map['ī'] = "i"; translit_map['į'] = "i"; translit_map['ı'] = "i"
translit_map['ĵ'] = "j"
translit_map['ķ'] = "k"
translit_map['ĺ'] = "l"; translit_map['ľ'] = "l"; translit_map['ł'] = "l"
translit_map['ñ'] = "n"; translit_map['ń'] = "n"; translit_map['ņ'] = "n"; translit_map['ŋ'] = "n"
translit_map['ó'] = "o"; translit_map['ò'] = "o"; translit_map['ö'] = "oe"; translit_map['ô'] = "o"
translit_map['õ'] = "o"; translit_map['ō'] = "o"; translit_map['ő'] = "o"
translit_map['ř'] = "r"; translit_map['ŕ'] = "r"
translit_map['ś'] = "s"; translit_map['š'] = "s"; translit_map['ş'] = "s"; translit_map['ß'] = "ss"
translit_map['ţ'] = "t"; translit_map['ť'] = "t"; translit_map['þ'] = "th"
translit_map['ú'] = "u"; translit_map['ù'] = "u"; translit_map['ü'] = "ue"; translit_map['û'] = "u"
translit_map['ū'] = "u"; translit_map['ů'] = "u"; translit_map['ų'] = "u"; translit_map['ű'] = "u"
translit_map['ý'] = "y"; translit_map['ÿ'] = "y"
translit_map['ž'] = "z"; translit_map['ź'] = "z"; translit_map['ż'] = "z"

# Latin Extended & Medieval (subset you had)
translit_map['ȝ'] = "y"
translit_map['ð'] = "th"
translit_map['ĸ'] = "k"

# Japanese long vowel mark
translit_map['ー'] = ""

# (Other large blocks from your JL are kept as comments for parity)

# -- Transliteration & normalization (as in your JL) --
function transliterate_char(c::Char)
    c_lower = lowercase(c)
    if haskey(translit_map, c_lower)
        return translit_map[c_lower]
    else
        base_chars = Unicode.normalize(string(c), :NFD)
        base_char = first(base_chars)
        if 'a' <= base_char <= 'z'
            return string(base_char)
        else
            return ""  # omit unknown chars
        end
    end
end

function transliterate_word(word::AbstractString)
    mapped_parts = (transliterate_char(c) for c in word)
    return lowercase(join(mapped_parts))
end

function regularize_name(name::AbstractString)
    lower = lowercase(name)
    ascii = transliterate_word(lower)
    return replace(ascii, r"[^a-z]" => "")
end

function read_words(filename::String)
    open(filename, "r") do f
        words = String[]
        for line in eachline(f)
            line = strip(line)
            if !isempty(line)
                for sep in ["_", "/", ":", "\\", " ", "-", "–", "—", "‒", "⸺", "⸻"]
                    line = replace(line, sep => "\n")
                end
                for subword in split(line, '\n')
                    if !isempty(subword)
                        push!(words, regularize_name(subword))
                    end
                end
            end
        end
        return words
    end
end

# === Cube-spellability checker (your bitmask + backtracking) ===
function can_spell(word::AbstractString, cubes::Vector{Set{Char}})
    alphabet = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q',
                'r','s','t','u','v','w','x','y','z']
    nn = length(alphabet)
    n_cubes = length(cubes)
    if n_cubes > 64
        error("Too many cubes for UInt64 bitmask optimization; adjust code for more cubes")
    end

    letter_to_idx = Dict{Char, Int}(c => i for (i,c) in enumerate(alphabet))

    letters = collect(word)
    n_letters = length(letters)
    if n_letters > n_cubes
        return false
    end

    # frequency array
    freq_word = zeros(Int, nn)
    for c in letters
        idx = get(letter_to_idx, c, 0)
        if idx == 0
            return false
        end
        freq_word[idx] += 1
    end

    # cube membership masks
    letter_masks = zeros(UInt64, nn)
    for (cube_i, cube) in enumerate(cubes)
        bit = UInt64(1) << (cube_i - 1)
        for c in cube
            idx = get(letter_to_idx, c, 0)
            if idx != 0
                letter_masks[idx] |= bit
            end
        end
    end

    # early check: required multiplicity vs number of cubes that contain letter
    for i in 1:nn
        if freq_word[i] > count_ones(letter_masks[i])
            return false
        end
    end

    # expand and sort letters by increasing availability (harder first)
    letters_indices = Int[]
    for i in 1:nn
        append!(letters_indices, fill(i, freq_word[i]))
    end
    sort!(letters_indices, by = i -> count_ones(letter_masks[i]))

    # precompute cube lists for each letter
    letter_cubes = Vector{Vector{Int}}(undef, nn)
    for i in 1:nn
        cubes_list = Int[]
        mask = letter_masks[i]
        while mask != 0
            push!(cubes_list, trailing_zeros(mask) + 1)
            mask &= mask - 1
        end
        sort!(cubes_list)
        letter_cubes[i] = cubes_list
    end

    MAX_PARALLEL_DEPTH = 2
    found = Atomic{Bool}(false)

    function backtrack(pos::Int, used_mask::UInt64, depth::Int, min_cube::Int)
        if found[]; return true; end
        if pos > length(letters_indices)
            found[] = true
            return true
        end

        letter_idx = letters_indices[pos]
        cubes_for_letter = letter_cubes[letter_idx]

        for cube_i in cubes_for_letter
            if cube_i < min_cube
                continue
            end
            bit = UInt64(1) << (cube_i - 1)
            if (used_mask & bit) == 0
                next_min_cube = if pos < length(letters_indices) && letters_indices[pos+1] == letter_idx
                    cube_i + 1
                else
                    1
                end
                if depth <= MAX_PARALLEL_DEPTH
                    t = @spawn backtrack(pos+1, used_mask | bit, depth+1, next_min_cube)
                    if fetch(t); return true; end
                else
                    if backtrack(pos+1, used_mask | bit, depth+1, next_min_cube)
                        return true
                    end
                end
            end
        end
        return false
    end

    return backtrack(1, zero(UInt64), 1, 1)
end

# === Filters (preserved) ===
function remove_roman_numerals(words::Vector{String})
    roman_regex = r"^m{0,4}(cm|cd|d?c{0,4})(xc|xl|l?x{0,4})(ix|iv|v?i{0,4})$"
    return filter(w -> !(occursin(roman_regex, lowercase(w)) && length(w) <= 10), words)
end

function remove_long_words(words::Vector{String}, max_length::Int=26)
    blocked = Set([
        "wqxga","boffff","fffipp","fufufu","javafx","wwmccs","sqvabb","faxvax","ssbbws","ixxviii","wwedded","sksksksk","jjws","ffqa","whqs","sksksk","abbaabba","abababcc","boffffs","wqhd","hmmwvs"
    ])
    #words = remove_roman_numerals(words)
    words = filter(w -> (length(w) <= max_length || w == "abcdefghijklmnopqrstuvwxyz") && !(w in blocked), words)
    return unique(words)
end

# === Simple analyzer for these fixed cubes ===
function analyze_with_fixed_cubes(words_to_check::Vector{String}, N::Int)
    # Only consider words with length <= N (like your runs)
    target = filter(w -> length(w) <= N, words_to_check)
    # Compute spellability
    spellable = Vector{String}()
    not_spellable = Vector{String}()
    for w in target
        (can_spell(w, cubes) ? push!(spellable, w) : push!(not_spellable, w))
    end
    sort!(spellable, by=length)
    sort!(not_spellable, by=length)
    return spellable, not_spellable
end

# --- dump helper (no deps) ---
function dump_words(path::AbstractString, ws::Vector{String})
    open(path, "w") do io
        for w in ws
            println(io, w)
        end
    end
end


# === Main: load **the same files** and check spellability on the provided cubes ===
function main()

    words = String[]
    # Bundled public-domain / permissively-licensed English word lists
    # (see DATA_SOURCES.md). Additional corpora fetched via download_scripts/
    # can be appended here once the resulting files are present in ./data/.
    append!(words, read_words("./data/webster.txt"))
    append!(words, read_words("./data/enable1.txt"))
    append!(words, read_words("./data/wordnet.txt"))
    append!(words, read_words("./data/nltk_words.txt"))

    words = sort(words, by=length, rev=false)
    words = unique(words)
    words_to_check = words

    # Apply the same filters (long-word + roman + blocklist)
    words_to_check = remove_long_words(words_to_check, 26)
    dump_words("jl_checked.txt", words_to_check)
    @info "Wrote jl_checked.txt" count=length(words_to_check)

    # === Verify spellability on the provided cubes ===
    N = 26
    spellable, not_spellable = analyze_with_fixed_cubes(words_to_check, N)

    println("Total words considered (len ≤ $N): ", length(spellable) + length(not_spellable))
    println("Spellable: ", length(spellable))
    println("Not spellable: ", length(not_spellable))

    for w in not_spellable 
        println(w)
    end

end

if abspath(PROGRAM_FILE) == @__FILE__
    main()
end

