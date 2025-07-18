# Changelog for egui_plot
All notable changes to the `egui_plot` integration will be noted in this file.

This file is updated upon each release.
Changes since the last release can be found at <https://github.com/emilk/egui_plot/compare/latest...HEAD> or by running the `scripts/generate_changelog.py` script.

## 0.33.0 - 2025-07-11

Full diff at https://github.com/emilk/egui_plot/compare/0.32.0..HEAD

#### PRs
* Update egui to 0.32 [#124](https://github.com/emilk/egui_plot/pull/124) by [@lucasmerlin](https://github.com/lucasmerlin)
* Fix dynamic bounds modifications [#96](https://github.com/emilk/egui_plot/pull/96) by [@emilk](https://github.com/emilk)
* Add `PlotUi::set_plot_bounds_x` and `set_plot_bounds_y` [#110](https://github.com/emilk/egui_plot/pull/110) by [@emilk](https://github.com/emilk)
* Allow zooming one axis by dragging it [#102](https://github.com/emilk/egui_plot/pull/102) by [@damageboy](https://github.com/damageboy)
* Add a background behind all text that is shown on hover [#112](https://github.com/emilk/egui_plot/pull/112) by [@emilk](https://github.com/emilk)
* Fix plot axis sometimes being way too thick [#113](https://github.com/emilk/egui_plot/pull/113) by [@emilk](https://github.com/emilk)
* Bump Rust Version and fix Plot Item Visibility in Demo [#104](https://github.com/emilk/egui_plot/pull/104) by [@bircni](https://github.com/bircni)
* Make circles in legend slightly smaller [#114](https://github.com/emilk/egui_plot/pull/114) by [@emilk](https://github.com/emilk)
* Show the hover-info about a plot using the `egui::Tooltip` API [#115](https://github.com/emilk/egui_plot/pull/115) by [@emilk](https://github.com/emilk)
* Add small margin to the axis tick labels [#117](https://github.com/emilk/egui_plot/pull/117) by [@emilk](https://github.com/emilk)
* Gradient line support [#83](https://github.com/emilk/egui_plot/pull/83) by [@sapessi](https://github.com/sapessi)
* Add optional legend title [#105](https://github.com/emilk/egui_plot/pull/105) by [@bircni](https://github.com/bircni)
* Fix BoxPlot legend [#97](https://github.com/emilk/egui_plot/pull/97) by [@mkalte666](https://github.com/mkalte666)
* When dragging the axis to zoom, zoom in where the drag started [871d400](https://github.com/emilk/egui_plot/commit/871d40053a1a2f270ea7e2e7112ef3c0b4ba71e6)
* Update MSRV to 1.85 and bump rust edition to 2024 [#116](https://github.com/emilk/egui_plot/pull/116) by [@emilk](https://github.com/emilk)
* Update to Rust 1.84 and enable many clippy lints [#107](https://github.com/emilk/egui_plot/pull/107) by [@abey79](https://github.com/abey79)
* Fix bad demo app name and icon [f835c51](https://github.com/emilk/egui_plot/commit/f835c51da82764c2b144b0f734ce4d212b538769)


## 0.32.1 - 2025-04-07

Full diff at https://github.com/emilk/egui_plot/compare/0.32.0..HEAD

#### PRs
* Fix dynamic bounds modifications [#96](https://github.com/emilk/egui_plot/pull/96) by [@emilk](https://github.com/emilk)


## 0.32.0 - 2025-04-07

Full diff at https://github.com/emilk/egui_plot/compare/0.31.0..HEAD

#### PRs
* `PlotResponse::hovered_plot_item` also set when hovering the legend [#81](https://github.com/emilk/egui_plot/pull/81) by [@Wumpf](https://github.com/Wumpf)
* Plot items now require an id [#82](https://github.com/emilk/egui_plot/pull/82) by [@Wumpf](https://github.com/Wumpf)
* Fix include_{xy} issue when auto bounds is off [#74](https://github.com/emilk/egui_plot/pull/74) by [@joaofl](https://github.com/joaofl)
* Provide new functions to specify the default bounds [#90](https://github.com/emilk/egui_plot/pull/90) by [@irevoire](https://github.com/irevoire)



## 0.31.0 - 2025-02-05

Full diff at https://github.com/emilk/egui_plot/compare/0.30.0..HEAD

#### PRs
* Allow borrowing plot points via `PlotPoints::Borrowed` [#64](https://github.com/emilk/egui_plot/pull/64) by [@mo8it](https://github.com/mo8it) and [@bircni](https://github.com/bircni)
* Add `insertion_order` and `color_conflict_handling` to `Legend` [#65](https://github.com/emilk/egui_plot/pull/65) by [@Zoxc](https://github.com/Zoxc) and [@bircni](https://github.com/bircni)
* Allow Plot::link_cursor to accept `impl Into<Vec2b>` [#66](https://github.com/emilk/egui_plot/pull/66) by [@jetuk](https://github.com/jetuk)
* Axis: fix label thickness [#68](https://github.com/emilk/egui_plot/pull/68) by [@jordens](https://github.com/jordens)
* Update to egui 0.31.0 [#72](https://github.com/emilk/egui_plot/pull/72) by [@Wumpf](https://github.com/Wumpf)
* Update MSRV to Rust 1.81 [#69](https://github.com/emilk/egui_plot/pull/69) by [@emilk](https://github.com/emilk)

## 0.30.0 - 2024-12-17

Full diff at https://github.com/emilk/egui_plot/compare/0.29.0..HEAD

#### PRs
* Update to egui `0.30.0`, MSRV to 1.80 [#59](https://github.com/emilk/egui_plot/pull/59) by [@bircni](https://github.com/bircni)
* Allow setting a line's fill area's alpha channel [#34](https://github.com/emilk/egui_plot/pull/34) by [@maxded](https://github.com/maxded)
* Use `Vec2b` in parameters [#43](https://github.com/emilk/egui_plot/pull/43) by [@bircni](https://github.com/bircni)
* Fix axis thickness for multiple X or Y axes [#60](https://github.com/emilk/egui_plot/pull/60) by [@raymanfx](https://github.com/raymanfx)
* Fix axis labels overlap with axis ticks [#57](https://github.com/emilk/egui_plot/pull/57) by [@mkalte666](https://github.com/mkalte666)
* Add `PlotUi::add_item(Box<dyn PlotItem>)` [#51](https://github.com/emilk/egui_plot/pull/51) by [@freeformstu](https://github.com/freeformstu)
* Implement custom ruler color for Plot [#47](https://github.com/emilk/egui_plot/pull/47) by [@gweisert](https://github.com/gweisert)


## 0.29.0 - 2024-09-26
* Update to egui 0.29 [#48](https://github.com/emilk/egui_plot/pull/48) by [@emilk](https://github.com/emilk)


## 0.28.1 - 2024-07-05
Nothing new


## 0.28.0 - 2024-07-03
### ⭐ Added
* Hide all other series when alt-clicking in the legend [#4549](https://github.com/emilk/egui/pull/4549) by [@abey79](https://github.com/abey79)

### 🔧 Changed
* `Plot::Items:allow_hover` give possibility to masked the interaction on hovered item [#2558](https://github.com/emilk/egui/pull/2558) by [@haricot](https://github.com/haricot)
* Expose `ClosestElem` and `PlotConfig` [#4380](https://github.com/emilk/egui/pull/4380) by [@rmburg](https://github.com/rmburg)
* Introduce lifetime to `egui_plot::Plot` to replace `'static` fields [#4435](https://github.com/emilk/egui/pull/4435) by [@Fabus1184](https://github.com/Fabus1184)
* Plot now respects the `interact_radius` set in the UI's style [#4520](https://github.com/emilk/egui/pull/4520) by [@YgorSouza](https://github.com/YgorSouza)
* Improve behavior of plot auto-bounds with reduced data [#4632](https://github.com/emilk/egui/pull/4632) by [@abey79](https://github.com/abey79)
* Improve default formatter of tick-marks [#4738](https://github.com/emilk/egui/pull/4738) by [@emilk](https://github.com/emilk)

### 🐛 Fixed
* Disable interaction for `ScrollArea` and `Plot` when UI is disabled [#4457](https://github.com/emilk/egui/pull/4457) by [@varphone](https://github.com/varphone)
* Make sure plot size is positive [#4429](https://github.com/emilk/egui/pull/4429) by [@rustbasic](https://github.com/rustbasic)
* Use `f64` for translate [#4637](https://github.com/emilk/egui/pull/4637) by [@Its-Just-Nans](https://github.com/Its-Just-Nans)
* Clamp plot zoom values to valid range [#4695](https://github.com/emilk/egui/pull/4695) by [@Its-Just-Nans](https://github.com/Its-Just-Nans)
* Fix plot bounds of empty plots [#4741](https://github.com/emilk/egui/pull/4741) by [@emilk](https://github.com/emilk)


## 0.27.2 - 2024-04-02
* Allow zoom/pan a plot as long as it contains the mouse cursor [#4292](https://github.com/emilk/egui/pull/4292)
* Prevent plot from resetting one axis while zooming/dragging the other [#4252](https://github.com/emilk/egui/pull/4252) (thanks [@YgorSouza](https://github.com/YgorSouza)!)
* egui_plot: Fix the same plot tick label being painted multiple times [#4307](https://github.com/emilk/egui/pull/4307)


## 0.27.1 - 2024-03-29
* Nothing new


## 0.27.0 - 2024-03-26
* Add `sense` option to `Plot` [#4052](https://github.com/emilk/egui/pull/4052) (thanks [@AmesingFlank](https://github.com/AmesingFlank)!)
* Plot widget - allow disabling scroll for x and y separately [#4051](https://github.com/emilk/egui/pull/4051) (thanks [@YgorSouza](https://github.com/YgorSouza)!)
* Fix panic when the base step size is set to 0 [#4078](https://github.com/emilk/egui/pull/4078) (thanks [@abey79](https://github.com/abey79)!)
* Expose `PlotGeometry` in public API [#4193](https://github.com/emilk/egui/pull/4193) (thanks [@dwuertz](https://github.com/dwuertz)!)


## 0.26.2 - 2024-02-14
* Nothing new


## 0.26.1 - 2024-02-11
* Nothing new


## 0.26.0 - 2024-02-05
* Make `egui_plot::PlotMemory` public [#3871](https://github.com/emilk/egui/pull/3871)
* Customizable spacing of grid and axis label spacing [#3896](https://github.com/emilk/egui/pull/3896)
* Change default plot line thickness from 1.0 to 1.5 [#3918](https://github.com/emilk/egui/pull/3918)
* Automatically expand plot axes thickness to fit their labels [#3921](https://github.com/emilk/egui/pull/3921)
* Plot items now have optional id which is returned in the plot's response when hovered [#3920](https://github.com/emilk/egui/pull/3920) (thanks [@Wumpf](https://github.com/Wumpf)!)
* Parallel tessellation with opt-in `rayon` feature [#3934](https://github.com/emilk/egui/pull/3934)
* Make `egui_plot::PlotItem` a public trait [#3943](https://github.com/emilk/egui/pull/3943)
* Fix clip rect for plot items [#3955](https://github.com/emilk/egui/pull/3955) (thanks [@YgorSouza](https://github.com/YgorSouza)!)


## 0.25.0 - 2024-01-08
* Fix plot auto-bounds unset by default [#3722](https://github.com/emilk/egui/pull/3722) (thanks [@abey79](https://github.com/abey79)!)
* Add methods to zoom a `Plot` programmatically [#2714](https://github.com/emilk/egui/pull/2714) (thanks [@YgorSouza](https://github.com/YgorSouza)!)
* Add a public API for overriding plot legend traces' visibilities [#3534](https://github.com/emilk/egui/pull/3534) (thanks [@jayzhudev](https://github.com/jayzhudev)!)


## 0.24.1 - 2024-12-03
* Fix plot auto-bounds default [#3722](https://github.com/emilk/egui/pull/3722) (thanks [@abey79](https://github.com/abey79)!)


## 0.24.0 - 2023-11-23
* Add `emath::Vec2b`, replacing `egui_plot::AxisBools` [#3543](https://github.com/emilk/egui/pull/3543)
* Add `auto_bounds/set_auto_bounds` to `PlotUi` [#3587](https://github.com/emilk/egui/pull/3587) [#3586](https://github.com/emilk/egui/pull/3586) (thanks [@abey79](https://github.com/abey79)!)
* Update MSRV to Rust 1.72 [#3595](https://github.com/emilk/egui/pull/3595)


## 0.23.0 - 2023-09-27 - Initial release, after being forked out from `egui`
* Draw axis labels and ticks outside of plotting window [#2284](https://github.com/emilk/egui/pull/2284) (thanks [@JohannesProgrammiert](https://github.com/JohannesProgrammiert)!)
* Add `PlotUi::response()` to replace `plot_clicked()` etc [#3223](https://github.com/emilk/egui/pull/3223)
* Add rotation feature to plot images [#3121](https://github.com/emilk/egui/pull/3121) (thanks [@ThundR67](https://github.com/ThundR67)!)
* Plot items: Image rotation and size in plot coordinates, polygon fill color [#3182](https://github.com/emilk/egui/pull/3182) (thanks [@s-nie](https://github.com/s-nie)!)
* Add method to specify `tip_size` of plot arrows [#3138](https://github.com/emilk/egui/pull/3138) (thanks [@nagua](https://github.com/nagua)!)
* Better handle additive colors in plots [#3387](https://github.com/emilk/egui/pull/3387)
* Fix auto_bounds when only one axis has restricted navigation [#3171](https://github.com/emilk/egui/pull/3171) (thanks [@KoffeinFlummi](https://github.com/KoffeinFlummi)!)
* Fix plot formatter not taking closures [#3260](https://github.com/emilk/egui/pull/3260) (thanks [@Wumpf](https://github.com/Wumpf)!)
