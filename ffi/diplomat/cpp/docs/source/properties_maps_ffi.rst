``properties_maps::ffi``
========================

.. cpp:class:: ICU4XCodePointMapData16

    An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.

    For properties whose values fit into 16 bits.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/index.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointMapData16, ICU4XError> try_get_script(const ICU4XDataProvider& provider)

        Gets a map for Unicode property Script from a :cpp:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_script.html>`__ for more information.


    .. cpp:function:: uint16_t get(char32_t cp) const

        Gets the value for a code point.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get>`__ for more information.


.. cpp:class:: ICU4XCodePointMapData8

    An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.

    For properties whose values fit into 8 bits.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/index.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointMapData8, ICU4XError> try_get_general_category(const ICU4XDataProvider& provider)

        Gets a map for Unicode property General_Category from a :cpp:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_general_category.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointMapData8, ICU4XError> try_get_bidi_class(const ICU4XDataProvider& provider)

        Gets a map for Unicode property Bidi_Class from a :cpp:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_bidi_class.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointMapData8, ICU4XError> try_get_east_asian_width(const ICU4XDataProvider& provider)

        Gets a map for Unicode property East_Asian_Width from a :cpp:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_east_asian_width.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointMapData8, ICU4XError> try_get_line_break(const ICU4XDataProvider& provider)

        Gets a map for Unicode property Line_Break from a :cpp:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_line_break.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointMapData8, ICU4XError> try_grapheme_cluster_break(const ICU4XDataProvider& provider)

        Gets a map for Unicode property Grapheme_Cluster_Break from a :cpp:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_grapheme_cluster_break.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointMapData8, ICU4XError> try_get_word_break(const ICU4XDataProvider& provider)

        Gets a map for Unicode property Word_Break from a :cpp:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_word_break.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointMapData8, ICU4XError> try_get_sentence_break(const ICU4XDataProvider& provider)

        Gets a map for Unicode property Sentence_Break from a :cpp:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_sentence_break.html>`__ for more information.


    .. cpp:function:: uint8_t get(char32_t cp) const

        Gets the value for a code point.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get>`__ for more information.

