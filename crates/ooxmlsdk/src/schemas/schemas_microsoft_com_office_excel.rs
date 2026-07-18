//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ClipboardFormatValues {
  #[sdk(rename = "PictOld")]
  #[default]
  PictureOld,
  #[sdk(rename = "Pict")]
  Picture,
  #[sdk(rename = "Bitmap")]
  Bitmap,
  #[sdk(rename = "PictPrint")]
  PicturePrint,
  #[sdk(rename = "PictScreen")]
  PictureScreen,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ObjectValues {
  #[sdk(rename = "Button")]
  #[default]
  Button,
  #[sdk(rename = "Checkbox")]
  Checkbox,
  #[sdk(rename = "Dialog")]
  Dialog,
  #[sdk(rename = "Drop")]
  Drop,
  #[sdk(rename = "Edit")]
  Edit,
  #[sdk(rename = "GBox")]
  GroupBox,
  #[sdk(rename = "Label")]
  Label,
  #[sdk(rename = "LineA")]
  AuditingLine,
  #[sdk(rename = "List")]
  List,
  #[sdk(rename = "Movie")]
  Movie,
  #[sdk(rename = "Note")]
  Note,
  #[sdk(rename = "Pict")]
  Picture,
  #[sdk(rename = "Radio")]
  Radio,
  #[sdk(rename = "RectA")]
  AuditingRectangle,
  #[sdk(rename = "Scroll")]
  Scroll,
  #[sdk(rename = "Spin")]
  Spin,
  #[sdk(rename = "Shape")]
  Shape,
  #[sdk(rename = "Group")]
  Group,
  #[sdk(rename = "Rect")]
  Rectangle,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BooleanEntryWithBlankValues {
  #[sdk(rename = "True")]
  #[default]
  True,
  #[sdk(rename = "t")]
  T,
  #[sdk(rename = "False")]
  False,
  #[sdk(rename = "f")]
  F,
  #[sdk(rename = "")]
  Empty,
}
/// Attached Object Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xvml:ClientData")]
pub struct ClientData {
  /// Object type
  #[sdk(attr(qname = ":ObjectType"))]
  pub object_type: ObjectValues,
  #[sdk(
        choice(
            text_child(variant = MoveWithCells, enum, qname = "xvml:MoveWithCells"),
            text_child(variant = ResizeWithCells, enum, qname = "xvml:SizeWithCells"),
            text_child(variant = Anchor, qname = "xvml:Anchor"),
            text_child(variant = Locked, enum, qname = "xvml:Locked"),
            text_child(variant = DefaultSize, enum, qname = "xvml:DefaultSize"),
            text_child(variant = PrintObject, enum, qname = "xvml:PrintObject"),
            text_child(variant = Disabled, enum, qname = "xvml:Disabled"),
            text_child(variant = AutoFill, enum, qname = "xvml:AutoFill"),
            text_child(variant = AutoLine, enum, qname = "xvml:AutoLine"),
            text_child(variant = AutoSizePicture, enum, qname = "xvml:AutoPict"),
            text_child(variant = FormulaMacro, qname = "xvml:FmlaMacro"),
            text_child(variant = HorizontalTextAlignment, qname = "xvml:TextHAlign"),
            text_child(variant = VerticalTextAlignment, qname = "xvml:TextVAlign"),
            text_child(variant = LockText, enum, qname = "xvml:LockText"),
            text_child(variant = JustifyLastLine, enum, qname = "xvml:JustLastX"),
            text_child(variant = SecretEdit, enum, qname = "xvml:SecretEdit"),
            text_child(variant = DefaultButton, enum, qname = "xvml:Default"),
            text_child(variant = HelpButton, enum, qname = "xvml:Help"),
            text_child(variant = CancelButton, enum, qname = "xvml:Cancel"),
            text_child(variant = DismissButton, enum, qname = "xvml:Dismiss"),
            text_child(variant = AcceleratorPrimary, qname = "xvml:Accel"),
            text_child(variant = AcceleratorSecondary, qname = "xvml:Accel2"),
            text_child(variant = CommentRowTarget, qname = "xvml:Row"),
            text_child(variant = CommentColumnTarget, qname = "xvml:Column"),
            text_child(variant = Visible, enum, qname = "xvml:Visible"),
            text_child(variant = RowHidden, enum, qname = "xvml:RowHidden"),
            text_child(variant = ColumnHidden, enum, qname = "xvml:ColHidden"),
            text_child(variant = InputValidationType, qname = "xvml:VTEdit"),
            text_child(variant = MultiLine, enum, qname = "xvml:MultiLine"),
            text_child(variant = VerticalScrollBar, enum, qname = "xvml:VScroll"),
            text_child(variant = ValidIds, enum, qname = "xvml:ValidIds"),
            text_child(variant = FormulaRange, qname = "xvml:FmlaRange"),
            text_child(variant = MinDropDownWidth, qname = "xvml:WidthMin"),
            text_child(variant = SelectionEntry, qname = "xvml:Sel"),
            text_child(
                variant = Disable3DForListBoxAndDropDown,
                enum,
                qname = "xvml:NoThreeD2"
            ),
            text_child(variant = SelectionType, qname = "xvml:SelType"),
            text_child(variant = MultiSelections, qname = "xvml:MultiSel"),
            text_child(variant = ListBoxCallbackType, qname = "xvml:LCT"),
            text_child(variant = ListItem, qname = "xvml:ListItem"),
            text_child(variant = DropStyle, qname = "xvml:DropStyle"),
            text_child(variant = Colored, enum, qname = "xvml:Colored"),
            text_child(variant = DropLines, qname = "xvml:DropLines"),
            text_child(variant = Checked, qname = "xvml:Checked"),
            text_child(variant = FormulaLink, qname = "xvml:FmlaLink"),
            text_child(variant = FormulaPicture, qname = "xvml:FmlaPict"),
            text_child(variant = Disable3D, enum, qname = "xvml:NoThreeD"),
            text_child(variant = FirstButton, enum, qname = "xvml:FirstButton"),
            text_child(variant = FormulaGroup, qname = "xvml:FmlaGroup"),
            text_child(variant = ScrollBarPosition, qname = "xvml:Val"),
            text_child(variant = ScrollBarMin, qname = "xvml:Min"),
            text_child(variant = ScrollBarMax, qname = "xvml:Max"),
            text_child(variant = ScrollBarIncrement, qname = "xvml:Inc"),
            text_child(variant = ScrollBarPageIncrement, qname = "xvml:Page"),
            text_child(variant = HorizontalScrollBar, enum, qname = "xvml:Horiz"),
            text_child(variant = ScrollBarWidth, qname = "xvml:Dx"),
            text_child(variant = MapOcxControl, enum, qname = "xvml:MapOCX"),
            text_child(variant = ClipboardFormat, enum, qname = "xvml:CF"),
            text_child(variant = CameraObject, enum, qname = "xvml:Camera"),
            text_child(variant = RecalculateAlways, enum, qname = "xvml:RecalcAlways"),
            text_child(variant = AutoScaleFont, enum, qname = "xvml:AutoScale"),
            text_child(variant = DdeObject, enum, qname = "xvml:DDE"),
            text_child(variant = UiObject, enum, qname = "xvml:UIObj"),
            text_child(variant = ScriptText, qname = "xvml:ScriptText"),
            text_child(variant = ScriptExtended, qname = "xvml:ScriptExtended"),
            text_child(variant = ScriptLanguage, qname = "xvml:ScriptLanguage"),
            text_child(variant = ScriptLocation, qname = "xvml:ScriptLocation"),
            text_child(variant = FormulaTextBox, qname = "xvml:FmlaTxbx")
        )
    )]
  pub client_data_choice: Vec<ClientDataChoice>,
}
/// Move with Cells.
pub type MoveWithCells = BooleanEntryWithBlankValues;
/// Resize with Cells.
pub type ResizeWithCells = BooleanEntryWithBlankValues;
/// Lock Toggle.
pub type Locked = BooleanEntryWithBlankValues;
/// Default Size Toggle.
pub type DefaultSize = BooleanEntryWithBlankValues;
/// Print Toggle.
pub type PrintObject = BooleanEntryWithBlankValues;
/// Macro Disable Toggle.
pub type Disabled = BooleanEntryWithBlankValues;
/// AutoFill.
pub type AutoFill = BooleanEntryWithBlankValues;
/// AutoLine.
pub type AutoLine = BooleanEntryWithBlankValues;
/// Automatically Size.
pub type AutoSizePicture = BooleanEntryWithBlankValues;
/// Text Lock.
pub type LockText = BooleanEntryWithBlankValues;
/// East Asia Alignment Toggle.
pub type JustifyLastLine = BooleanEntryWithBlankValues;
/// Password Edit.
pub type SecretEdit = BooleanEntryWithBlankValues;
/// Default Button.
pub type DefaultButton = BooleanEntryWithBlankValues;
/// Help Button.
pub type HelpButton = BooleanEntryWithBlankValues;
/// Cancel Button.
pub type CancelButton = BooleanEntryWithBlankValues;
/// Dismiss Button.
pub type DismissButton = BooleanEntryWithBlankValues;
/// Comment Visibility Toggle.
pub type Visible = BooleanEntryWithBlankValues;
/// Comment's Row is Hidden.
pub type RowHidden = BooleanEntryWithBlankValues;
/// Comment's Column is Hidden.
pub type ColumnHidden = BooleanEntryWithBlankValues;
/// Multi-line.
pub type MultiLine = BooleanEntryWithBlankValues;
/// Vertical Scroll.
pub type VerticalScrollBar = BooleanEntryWithBlankValues;
/// Valid ID.
pub type ValidIds = BooleanEntryWithBlankValues;
/// Disable 3D.
pub type Disable3DForListBoxAndDropDown = BooleanEntryWithBlankValues;
/// Dropdown Color Toggle.
pub type Colored = BooleanEntryWithBlankValues;
/// Disable 3D.
pub type Disable3D = BooleanEntryWithBlankValues;
/// First Radio Button.
pub type FirstButton = BooleanEntryWithBlankValues;
/// Scroll Bar Orientation.
pub type HorizontalScrollBar = BooleanEntryWithBlankValues;
/// ActiveX Control.
pub type MapOcxControl = BooleanEntryWithBlankValues;
/// Camera Tool.
pub type CameraObject = BooleanEntryWithBlankValues;
/// Recalculation Toggle.
pub type RecalculateAlways = BooleanEntryWithBlankValues;
/// Font AutoScale.
pub type AutoScaleFont = BooleanEntryWithBlankValues;
/// Dynamic Data Exchange.
pub type DdeObject = BooleanEntryWithBlankValues;
/// UI Object Toggle.
pub type UiObject = BooleanEntryWithBlankValues;
/// Anchor.
pub type Anchor = crate::simple_type::StringValue;
/// Horizontal Text Alignment.
pub type HorizontalTextAlignment = crate::simple_type::StringValue;
/// Vertical Text Alignment.
pub type VerticalTextAlignment = crate::simple_type::StringValue;
/// List Items Source Range.
pub type FormulaRange = crate::simple_type::StringValue;
/// Selection Type.
pub type SelectionType = crate::simple_type::StringValue;
/// Multiple Selections.
pub type MultiSelections = crate::simple_type::StringValue;
/// Callback Type.
pub type ListBoxCallbackType = crate::simple_type::StringValue;
/// Non-linked List Item.
pub type ListItem = crate::simple_type::StringValue;
/// Dropdown Style.
pub type DropStyle = crate::simple_type::StringValue;
/// Linked Formula.
pub type FormulaLink = crate::simple_type::StringValue;
/// Camera Source Range.
pub type FormulaPicture = crate::simple_type::StringValue;
/// Linked Formula - Group Box.
pub type FormulaGroup = crate::simple_type::StringValue;
/// HTML Script Text.
pub type ScriptText = crate::simple_type::StringValue;
/// HTML Script Attributes.
pub type ScriptExtended = crate::simple_type::StringValue;
/// Text Formula.
pub type FormulaTextBox = crate::simple_type::StringValue;
/// Reference to Custom Function.
pub type FormulaMacro = crate::simple_type::StringValue;
/// Primary Keyboard Accelerator.
pub type AcceleratorPrimary = crate::simple_type::ByteValue;
/// Secondary Keyboard Accelerator.
pub type AcceleratorSecondary = crate::simple_type::ByteValue;
/// Comment Row Target.
pub type CommentRowTarget = crate::simple_type::IntegerValue;
/// Comment Column Target.
pub type CommentColumnTarget = crate::simple_type::IntegerValue;
/// Validation Type.
pub type InputValidationType = crate::simple_type::IntegerValue;
/// Minimum Width.
pub type MinDropDownWidth = crate::simple_type::IntegerValue;
/// Selected Entry.
pub type SelectionEntry = crate::simple_type::IntegerValue;
/// Dropdown Maximum Lines.
pub type DropLines = crate::simple_type::IntegerValue;
/// Checked.
pub type Checked = crate::simple_type::IntegerValue;
/// Scroll bar position.
pub type ScrollBarPosition = crate::simple_type::IntegerValue;
/// Scroll Bar Minimum.
pub type ScrollBarMin = crate::simple_type::IntegerValue;
/// Scroll Bar Maximum.
pub type ScrollBarMax = crate::simple_type::IntegerValue;
/// Scroll Bar Increment.
pub type ScrollBarIncrement = crate::simple_type::IntegerValue;
/// Scroll Bar Page Increment.
pub type ScrollBarPageIncrement = crate::simple_type::IntegerValue;
/// Scroll Bar Width.
pub type ScrollBarWidth = crate::simple_type::IntegerValue;
/// Clipboard Format.
pub type ClipboardFormat = ClipboardFormatValues;
/// HTML Script Language.
pub type ScriptLanguage = crate::simple_type::IntegerValue;
/// HTML Script Location.
pub type ScriptLocation = crate::simple_type::IntegerValue;
#[derive(Clone, Debug, PartialEq)]
pub enum ClientDataChoice {
  /// Move with Cells.
  MoveWithCells(MoveWithCells),
  /// Resize with Cells.
  ResizeWithCells(ResizeWithCells),
  /// Anchor.
  Anchor(Anchor),
  /// Lock Toggle.
  Locked(Locked),
  /// Default Size Toggle.
  DefaultSize(DefaultSize),
  /// Print Toggle.
  PrintObject(PrintObject),
  /// Macro Disable Toggle.
  Disabled(Disabled),
  /// AutoFill.
  AutoFill(AutoFill),
  /// AutoLine.
  AutoLine(AutoLine),
  /// Automatically Size.
  AutoSizePicture(AutoSizePicture),
  /// Reference to Custom Function.
  FormulaMacro(FormulaMacro),
  /// Horizontal Text Alignment.
  HorizontalTextAlignment(HorizontalTextAlignment),
  /// Vertical Text Alignment.
  VerticalTextAlignment(VerticalTextAlignment),
  /// Text Lock.
  LockText(LockText),
  /// East Asia Alignment Toggle.
  JustifyLastLine(JustifyLastLine),
  /// Password Edit.
  SecretEdit(SecretEdit),
  /// Default Button.
  DefaultButton(DefaultButton),
  /// Help Button.
  HelpButton(HelpButton),
  /// Cancel Button.
  CancelButton(CancelButton),
  /// Dismiss Button.
  DismissButton(DismissButton),
  /// Primary Keyboard Accelerator.
  AcceleratorPrimary(AcceleratorPrimary),
  /// Secondary Keyboard Accelerator.
  AcceleratorSecondary(AcceleratorSecondary),
  /// Comment Row Target.
  CommentRowTarget(CommentRowTarget),
  /// Comment Column Target.
  CommentColumnTarget(CommentColumnTarget),
  /// Comment Visibility Toggle.
  Visible(Visible),
  /// Comment's Row is Hidden.
  RowHidden(RowHidden),
  /// Comment's Column is Hidden.
  ColumnHidden(ColumnHidden),
  /// Validation Type.
  InputValidationType(InputValidationType),
  /// Multi-line.
  MultiLine(MultiLine),
  /// Vertical Scroll.
  VerticalScrollBar(VerticalScrollBar),
  /// Valid ID.
  ValidIds(ValidIds),
  /// List Items Source Range.
  FormulaRange(FormulaRange),
  /// Minimum Width.
  MinDropDownWidth(MinDropDownWidth),
  /// Selected Entry.
  SelectionEntry(SelectionEntry),
  /// Disable 3D.
  Disable3DForListBoxAndDropDown(Disable3DForListBoxAndDropDown),
  /// Selection Type.
  SelectionType(SelectionType),
  /// Multiple Selections.
  MultiSelections(MultiSelections),
  /// Callback Type.
  ListBoxCallbackType(ListBoxCallbackType),
  /// Non-linked List Item.
  ListItem(ListItem),
  /// Dropdown Style.
  DropStyle(DropStyle),
  /// Dropdown Color Toggle.
  Colored(Colored),
  /// Dropdown Maximum Lines.
  DropLines(DropLines),
  /// Checked.
  Checked(Checked),
  /// Linked Formula.
  FormulaLink(FormulaLink),
  /// Camera Source Range.
  FormulaPicture(FormulaPicture),
  /// Disable 3D.
  Disable3D(Disable3D),
  /// First Radio Button.
  FirstButton(FirstButton),
  /// Linked Formula - Group Box.
  FormulaGroup(FormulaGroup),
  /// Scroll bar position.
  ScrollBarPosition(ScrollBarPosition),
  /// Scroll Bar Minimum.
  ScrollBarMin(ScrollBarMin),
  /// Scroll Bar Maximum.
  ScrollBarMax(ScrollBarMax),
  /// Scroll Bar Increment.
  ScrollBarIncrement(ScrollBarIncrement),
  /// Scroll Bar Page Increment.
  ScrollBarPageIncrement(ScrollBarPageIncrement),
  /// Scroll Bar Orientation.
  HorizontalScrollBar(HorizontalScrollBar),
  /// Scroll Bar Width.
  ScrollBarWidth(ScrollBarWidth),
  /// ActiveX Control.
  MapOcxControl(MapOcxControl),
  /// Clipboard Format.
  ClipboardFormat(ClipboardFormat),
  /// Camera Tool.
  CameraObject(CameraObject),
  /// Recalculation Toggle.
  RecalculateAlways(RecalculateAlways),
  /// Font AutoScale.
  AutoScaleFont(AutoScaleFont),
  /// Dynamic Data Exchange.
  DdeObject(DdeObject),
  /// UI Object Toggle.
  UiObject(UiObject),
  /// HTML Script Text.
  ScriptText(ScriptText),
  /// HTML Script Attributes.
  ScriptExtended(ScriptExtended),
  /// HTML Script Language.
  ScriptLanguage(ScriptLanguage),
  /// HTML Script Location.
  ScriptLocation(ScriptLocation),
  /// Text Formula.
  FormulaTextBox(FormulaTextBox),
}
