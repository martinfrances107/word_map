# WORD MAP

Rust 2021 Edition.

A library: Given a list of words and weight produce a image like one of the examples below. The size of words in the cloud is determined by the weight. A integer between 1 and 10.

It in a early development stage.

## Examples

Can be found in the github repository associated with this crate.

<table>

<thead>

<th align="left" colspan="2">Description</th>

</thead>

<tbody align="left" style="vertical-align:top;">
<tr>
<td>

<strong>examples/blueprint</strong>

Generates a world cloud based on a number of (word, weight) pairs selected at random. As a confidence building exercise each block is displayed as a rectangle with the two definition points show as circles

</td>

<td>
<img src="https://raw.githubusercontent.com/martinfrances107/word_map/main/images/blueprint.svg" alt="A wordmap stylized as a Blueprint" title="A wordmap stylized as a Blueprint"/>
</td>

</tr>

<tr>
<td>

  <strong>examples/radial</strong>

  The library allow the definition of bounding rectangle limiting the placement of new text.

  For example this allow large text blocks to constrained to a central region while small text blocks can be placed over the full page.
</td>

<td>
  <img src="https://raw.githubusercontent.com/martinfrances107/word_map/main/images/radial.svg" alt="Large text blocks are centrally placed" title="Large text blocks are centrally placed"/>
</td>

</tr>

<tr>
<td>
  TODO: A website width a word cloud selected supplied by a text file
</td>

<td>
  IMG3
</td>

</tr>

</tbody>
</table>
