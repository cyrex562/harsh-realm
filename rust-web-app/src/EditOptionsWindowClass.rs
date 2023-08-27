// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.EditOptionsWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class EditOptionsWindowClass : WindowClass
  {
    pub BShowMapId: i32;
    pub BShowMapTextId: i32;
    pub BBackId: i32;
    pub BBackTextId: i32;
    pub BLTId: i32;
    pub BLTTextId: i32;
    pub BEditTextId: i32;
    pub BRoadId: i32;
    pub BRoadTextId: i32;
    pub BHisId: i32;
    pub BHisTextId: i32;
    pub BRiverID: i32;
    pub BRiverTextId: i32;
    pub BBridgeID: i32;
    pub BBridgeTextID: i32;
    pub BRegimeID: i32;
    pub BRegimeTextId: i32;
    pub BSFTypeID: i32;
    pub BSFTypeTextId: i32;
    pub BEditUnitID: i32;
    pub BEditUnitTextId: i32;
    pub bLocTypId: i32;
    pub BLocTypTextId: i32;
    pub BResetPaintId: i32;
    pub BDefaultStringsID: i32;
    pub BDefaultStringsTextId: i32;
    pub BPeopleID: i32;
    pub BPeopleTextId: i32;
    pub BLocationID: i32;
    pub BLocationTextId: i32;
    pub BItemTypeID: i32;
    pub BItemTypeTextId: i32;
    pub BEventTypeID: i32;
    pub BEventTypeTextId: i32;
    pub BConnectionID: i32;
    pub BConnectionTextId: i32;
    pub BActionID: i32;
    pub BActionTextID: i32;
    pub AddMapId: i32;
    pub RemoveMapId: i32;
    pub maploopid: i32;
     BDraw2Id: i32;
     BDraw2TextId: i32;
    pub BResID: i32;
    pub BResTextId: i32;
    pub BLibId: i32;
    pub BLibTextId: i32;
    pub bStringListId: i32;
    pub bStringListTextId: i32;
    pub opt1id: i32;
    pub opt2id: i32;
    pub opt3id: i32;
    pub opt4id: i32;
    pub opt5id: i32;
    pub opt6id: i32;
    pub pickid: i32;
     firstListId: i32;
     ListClass firstListObj;
     medListId: i32;
     ListClass medListObj;
     lastListId: i32;
     ListClass lastListObj;
     MapListId: i32;
     ListClass MapListObj;
     Shortsdesc: i32;
     detailnr: i32;
     bool warningshown;

    pub fn PopUpRefresh() => this.dostuff();

    pub EditOptionsWindowClass(ref tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 =  -1, let mut sy: i32 =  -1)
      : base(ref tGame, tGame.ScreenWidth, 100, 99, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      this.domenu();
      this.dostuff();
      this.game.EditObj.inSimpleEditor = false;
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (!this.game.EditObj.warningshown)
      {
        str: String = "ADVANCED EDITOR IS STILL IN BETA\r\n" + "Check the matrix forums and save your work often!";
        this.game.EditObj.PopupValue = 10;
        this.game.EditObj.QuestionText = str;
        this.game.EditObj.AnswerCount = 1;
        this.game.EditObj.AnswerText[1] = "Ok".to_owned();
        this.game.EditObj.warningshown = true;
        windowReturnClass.AddCommand(5, 10);
        this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub fn domenu()
    {
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.BUTTONKILL);
      this.BBackId = this.AddSubPart(ref tsubpart1, 10, 2, 32, 16, 1);
      let mut tsubpart2: SubPartClass =  TextPartClass::new("Main", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.BBackTextId = this.AddSubPart(ref tsubpart2, 45, 1, 30, 16, 0);
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.BShowMapId = this.AddSubPart(ref tsubpart3, 10, 22, 32, 16, 1);
      let mut tsubpart4: SubPartClass =  TextPartClass::new("Map", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.BShowMapTextId = this.AddSubPart(ref tsubpart4, 45, 21, 30, 16, 0);
      let mut tsubpart5: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.BLTId = this.AddSubPart(ref tsubpart5, 10, 42, 32, 16, 1);
      let mut tsubpart6: SubPartClass =  TextPartClass::new("Land", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.BLTTextId = this.AddSubPart(ref tsubpart6, 45, 41, 30, 16, 0);
      let mut tsubpart7: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.BRoadId = this.AddSubPart(ref tsubpart7, 10, 62, 32, 16, 1);
      let mut tsubpart8: SubPartClass =  TextPartClass::new("Road", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.BRoadTextId = this.AddSubPart(ref tsubpart8, 45, 61, 30, 16, 0);
      let mut tsubpart9: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.BRiverID = this.AddSubPart(ref tsubpart9, 85, 2, 32, 16, 1);
      let mut tsubpart10: SubPartClass =  TextPartClass::new("River", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.BRiverTextId = this.AddSubPart(ref tsubpart10, 120, 1, 30, 16, 0);
      let mut tsubpart11: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.BRegimeID = this.AddSubPart(ref tsubpart11, 85, 22, 32, 16, 1);
      let mut tsubpart12: SubPartClass =  TextPartClass::new("Reg", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.BRegimeTextId = this.AddSubPart(ref tsubpart12, 120, 21, 30, 16, 0);
      let mut tsubpart13: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.BSFTypeID = this.AddSubPart(ref tsubpart13, 85, 42, 32, 16, 1);
      let mut tsubpart14: SubPartClass =  TextPartClass::new("SFT", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.BSFTypeTextId = this.AddSubPart(ref tsubpart14, 120, 41, 30, 16, 0);
      let mut tsubpart15: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.BEditUnitID = this.AddSubPart(ref tsubpart15, 85, 62, 32, 16, 1);
      let mut tsubpart16: SubPartClass =  TextPartClass::new("Units", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.BEditUnitTextId = this.AddSubPart(ref tsubpart16, 120, 61, 30, 16, 0);
      let mut tsubpart17: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.BActionID = this.AddSubPart(ref tsubpart17, 85, 82, 32, 16, 1);
      let mut tsubpart18: SubPartClass =  TextPartClass::new("Cards", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.BActionTextID = this.AddSubPart(ref tsubpart18, 120, 81, 30, 16, 0);
      let mut tsubpart19: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.BDefaultStringsID = this.AddSubPart(ref tsubpart19, 160, 2, 32, 16, 1);
      let mut tsubpart20: SubPartClass =  TextPartClass::new("Setng", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.BDefaultStringsTextId = this.AddSubPart(ref tsubpart20, 195, 1, 30, 16, 0);
      let mut tsubpart21: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.bLocTypId = this.AddSubPart(ref tsubpart21, 160, 22, 32, 16, 1);
      let mut tsubpart22: SubPartClass =  TextPartClass::new("LT", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.BLocTypTextId = this.AddSubPart(ref tsubpart22, 195, 21, 30, 16, 0);
      let mut tsubpart23: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.BPeopleID = this.AddSubPart(ref tsubpart23, 160, 42, 32, 16, 1);
      let mut tsubpart24: SubPartClass =  TextPartClass::new("Ppl", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.BPeopleTextId = this.AddSubPart(ref tsubpart24, 195, 41, 30, 16, 0);
      let mut tsubpart25: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.BLocationID = this.AddSubPart(ref tsubpart25, 160, 62, 32, 16, 1);
      let mut tsubpart26: SubPartClass =  TextPartClass::new("Hex", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 40, 16, false);
      this.BLocationTextId = this.AddSubPart(ref tsubpart26, 195, 61, 40, 16, 0);
      let mut tsubpart27: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.bStringListId = this.AddSubPart(ref tsubpart27, 160, 82, 32, 16, 1);
      let mut tsubpart28: SubPartClass =  TextPartClass::new("String", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.bStringListTextId = this.AddSubPart(ref tsubpart28, 195, 81, 30, 16, 0);
      let mut tsubpart29: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.BEventTypeID = this.AddSubPart(ref tsubpart29, 235, 2, 32, 16, 1);
      let mut tsubpart30: SubPartClass =  TextPartClass::new("Event", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.BEventTypeTextId = this.AddSubPart(ref tsubpart30, 270, 1, 30, 16, 0);
      let mut tsubpart31: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.BBridgeID = this.AddSubPart(ref tsubpart31, 235, 42, 32, 16, 1);
      let mut tsubpart32: SubPartClass =  TextPartClass::new("Brdge", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.BBridgeTextID = this.AddSubPart(ref tsubpart32, 270, 41, 30, 16, 0);
      let mut tsubpart33: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.BHisId = this.AddSubPart(ref tsubpart33, 235, 82, 32, 16, 1);
      let mut tsubpart34: SubPartClass =  TextPartClass::new("His", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.BHisTextId = this.AddSubPart(ref tsubpart34, 270, 81, 30, 16, 0);
      let mut tsubpart35: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.BLibId = this.AddSubPart(ref tsubpart35, 310, 2, 32, 16, 1);
      let mut tsubpart36: SubPartClass =  TextPartClass::new("Lib", Font::new("Times New Roman", 12f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 16, false);
      this.BLibTextId = this.AddSubPart(ref tsubpart36, 345, 1, 30, 16, 0);
      let mut x: i32 =  870;
      let mut y1: i32 =  0;
      let mut tsubpart37: SubPartClass =  TextPartClass::new("SHORTKEYS FOR HIST.UNITS:", this.game.MarcFont10, 180, 15, false);
      this.AddSubPart(ref tsubpart37, x, y1, 180, 15, 1);
      let mut y2: i32 =  y1 + 16;
      let mut tsubpart38: SubPartClass =  TextPartClass::new("HOME = Place instance of a model his", this.game.MarcFont10, 180, 15, false);
      this.AddSubPart(ref tsubpart38, x, y2, 180, 15, 1);
      let mut y3: i32 =  y2 + 13;
      let mut tsubpart39: SubPartClass =  TextPartClass::new("PAGEUP = Place a non-model his", this.game.MarcFont10, 180, 15, false);
      this.AddSubPart(ref tsubpart39, x, y3, 180, 15, 1);
      let mut y4: i32 =  y3 + 13;
      let mut tsubpart40: SubPartClass =  TextPartClass::new("BACK= Remove his-link from unit", this.game.MarcFont10, 180, 15, false);
      this.AddSubPart(ref tsubpart40, x, y4, 180, 15, 1);
      let mut y5: i32 =  y4 + 13;
      let mut tsubpart41: SubPartClass =  TextPartClass::new("D0 = Select type for his of unit", this.game.MarcFont10, 180, 15, false);
      this.AddSubPart(ref tsubpart41, x, y5, 180, 15, 1);
      let mut y6: i32 =  y5 + 13;
      let mut tsubpart42: SubPartClass =  TextPartClass::new("D9 = Select shortname for his of unit", this.game.MarcFont10, 180, 15, false);
      this.AddSubPart(ref tsubpart42, x, y6, 180, 15, 1);
      let mut y7: i32 =  y6 + 13;
      let mut tsubpart43: SubPartClass =  TextPartClass::new("D8 = Select nato for his of unit", this.game.MarcFont10, 180, 15, false);
      this.AddSubPart(ref tsubpart43, x, y7, 180, 15, 1);
    }

    pub fn dostuff()
    {
      if (this.pickid > 0)
        this.RemoveSubPart(this.pickid);
      if (this.opt1id > 0)
        this.RemoveSubPart(this.opt1id);
      if (this.opt2id > 0)
        this.RemoveSubPart(this.opt2id);
      if (this.opt3id > 0)
        this.RemoveSubPart(this.opt3id);
      if (this.opt4id > 0)
        this.RemoveSubPart(this.opt4id);
      if (this.opt5id > 0)
        this.RemoveSubPart(this.opt5id);
      if (this.opt6id > 0)
        this.RemoveSubPart(this.opt6id);
      if (this.BEditTextId > 0)
        this.RemoveSubPart(this.BEditTextId);
      this.ClearMouse();
      this.NewBackGroundAndClearAll(this.game.ScreenWidth, 100, -1);
      this.FlagAll();
      this.detailnr = this.game.EditObj.MapSelected;
      bool flag;
      str: String;
      Left1: String;
      ttext: String;
      if (this.game.EditObj.PencilType > 0)
      {
        if (!(this.game.EditObj.PencilType == 3 | this.game.EditObj.PencilType == 10 | this.game.EditObj.PencilType == 1 | this.game.EditObj.PencilType == 9 | this.game.EditObj.PencilType == 11))
          flag = true;
        if (this.game.EditObj.PencilType == 1)
        {
          str = "Landsc# " + Conversion.Str( this.game.EditObj.PencilData1) + "," + Conversion.Str( this.game.EditObj.PencilData2);
          Left1 = this.game.Data.LandscapeTypeObj[this.game.EditObj.PencilData1].Name;
          ttext = "Left click to draw this landscape+sprite on a hex, right click to only select a hex.";
        }
        else if (this.game.EditObj.PencilType == 10)
        {
          str = "Spec.Lt# " + Conversion.Str( this.game.EditObj.PencilData1) + "," + Conversion.Str( this.game.EditObj.PencilData2);
          Left1 = this.game.EditObj.PencilData1 <= -1 ? "Remove Special" : this.game.Data.LandscapeTypeObj[this.game.EditObj.PencilData1].Name;
          ttext = "Left click to draw this landscape+sprite a a special on a hex, right click to only select a hex.";
        }
        else if (this.game.EditObj.PencilType == 2)
        {
          str = "Road# " + Conversion.Str( this.game.EditObj.PencilData1);
          Left1 = this.game.Data.RoadTypeObj[this.game.EditObj.PencilData1].Name;
          ttext = "First right click to select a hex, then left click on a neighbouring hex to draw a road between them.";
        }
        else if (this.game.EditObj.PencilType == 3)
        {
          str = "Reg# " + Conversion.Str( this.game.EditObj.PencilData1);
          Left1 = this.game.EditObj.PencilData1 <= this.game.Data.RegimeCounter ? this.game.Data.RegimeObj[this.game.EditObj.PencilData1].Name : "Neutral Hex";
          ttext = "Left click to draw this regime on a hex, right click just to select a hex, clicking twice results in hex becoming neutral again.";
        }
        else if (this.game.EditObj.PencilType == 4)
        {
          str = "LocTyp# " + Conversion.Str( this.game.EditObj.PencilData1);
          Left1 = this.game.Data.LocTypeObj[this.game.EditObj.PencilData1].Name;
          ttext = "Left click on a hex to place a location of this locationtype. Right click is only select.";
        }
        else if (this.game.EditObj.PencilType == 5)
        {
          str = "RiverTyp# " + Conversion.Str( this.game.EditObj.PencilData1);
          Left1 = this.game.Data.RiverTypeObj[this.game.EditObj.PencilData1].Name;
          ttext = "First right click to select a hex, then left click on a neighbouring hex to draw a river inbetween them.";
        }
        else if (this.game.EditObj.PencilType == 6)
        {
          str = "Bridge".to_owned();
          Left1 = "";
          ttext = "First right click to select a hex, then left click on a neighbouring hex to draw a bridge in between them.";
        }
        else if (this.game.EditObj.PencilType == 9)
        {
          str = "Slot# " + Conversion.Str( this.game.EditObj.PencilData1) + ", => " + Conversion.Str( this.game.EditObj.PencilData2);
          Left1 = "";
          ttext = "Left click to place this value on a hex, right click just to select a hex.";
        }
        else if (this.game.EditObj.PencilType == 11)
        {
          str = "LibVarSlot " + Conversion.Str( this.game.EditObj.PencilData1) + ", => " + Conversion.Str( this.game.EditObj.PencilData2);
          Left1 = "";
          ttext = "Left click to place this value on a hex, right click just to select a hex.";
        }
        else
        {
          str = "Pointer".to_owned();
          Left1 = "";
          ttext = "I hope you are having a good day!";
        }
      }
      let mut x: i32 =  735;
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      Left2: String = this.game.EditObj.PencilMode != 0 ? "Fill" : "Draw".to_owned();
      if (this.game.EditObj.PencilType == 0)
        Left2 = "None".to_owned();
      ref Graphics local1 = ref graphics;
      Rectangle rectangle1 = Rectangle::new(x, 5, 200, 10);
      let mut rect1_1: &Rectangle = &rectangle1
      Rectangle rectangle2 = Rectangle::new(x, 15, 120, 17);
      let mut rect2_1: &Rectangle = &rectangle2
      txt2_1: String = Left2;
      DrawMod.MakeFullBoxVic2(ref local1, rect1_1, "DRAW MODE", rect2_1, txt2_1);
      if (Operators.CompareString(Left2, "None", false) != 0)
      {
        ref Graphics local2 = ref graphics;
        rectangle2 = Rectangle::new(x, 35, 200, 10);
        let mut rect1_2: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x, 45, 120, 17);
        let mut rect2_2: &Rectangle = &rectangle1
        txt2_2: String = str;
        DrawMod.MakeFullBoxVic2(ref local2, rect1_2, "DRAW TYPE", rect2_2, txt2_2);
        if (Operators.CompareString(Left1, "", false) != 0)
        {
          ref Graphics local3 = ref graphics;
          rectangle2 = Rectangle::new(x, 65, 200, 10);
          let mut rect1_3: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, 75, 120, 17);
          let mut rect2_3: &Rectangle = &rectangle1
          txt2_3: String = Left1;
          DrawMod.MakeFullBoxVic2(ref local3, rect1_3, "DRAW TYPE NAME", rect2_3, txt2_3);
        }
        else
        {
          ref Graphics local4 = ref graphics;
          rectangle2 = Rectangle::new(x, 65, 200, 10);
          let mut rect1_4: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, 75, 120, 17);
          let mut rect2_4: &Rectangle = &rectangle1
          DrawMod.MakeFullBoxVic2(ref local4, rect1_4, "DRAW TYPE NAME", rect2_4, "");
        }
      }
      else
      {
        ref Graphics local5 = ref graphics;
        rectangle2 = Rectangle::new(x, 35, 200, 10);
        let mut rect1_5: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x, 45, 120, 17);
        let mut rect2_5: &Rectangle = &rectangle1
        DrawMod.MakeFullBoxVic2(ref local5, rect1_5, "DRAW TYPE", rect2_5, "");
        ref Graphics local6 = ref graphics;
        rectangle2 = Rectangle::new(x, 65, 200, 10);
        let mut rect1_6: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x, 75, 120, 17);
        let mut rect2_6: &Rectangle = &rectangle1
        DrawMod.MakeFullBoxVic2(ref local6, rect1_6, "DRAW TYPE NAME", rect2_6, "");
      }
      rectangle2 = Rectangle::new(x, 0, 120, 100);
      let mut trect: &Rectangle = &rectangle2
      this.AddMouse(ref trect, "", ttext);
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Pick Draw Type", 160, tBackbitmap: (ref this.OwnBitmap), bbx: 535, bby: 5, theight: 30);
      this.pickid = this.AddSubPart(ref tsubpart1, 535, 5, 160, 30, 1);
      if (Operators.CompareString(Left2, "Draw", false) == 0)
      {
        if (!flag)
        {
          let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Go to fill mode", 160, tBackbitmap: (ref this.OwnBitmap), bbx: 535, bby: 35, theight: 30);
          this.opt2id = this.AddSubPart(ref tsubpart2, 535, 35, 160, 30, 1);
        }
        else
        {
          let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Go to fill mode", 160, tBackbitmap: (ref this.OwnBitmap), bbx: 535, bby: 35, tinactive: true, theight: 30);
          this.opt4id = this.AddSubPart(ref tsubpart3, 535, 35, 160, 30, 0);
        }
      }
      else if (Operators.CompareString(Left2, "Fill", false) == 0)
      {
        let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Go to draw mode", 160, tBackbitmap: (ref this.OwnBitmap), bbx: 535, bby: 35, theight: 30);
        this.opt1id = this.AddSubPart(ref tsubpart4, 535, 35, 160, 30, 1);
      }
      if (Operators.CompareString(Left2, "None", false) != 0)
      {
        let mut tsubpart5: SubPartClass =  new TextButtonPartClass("End drawing mode", 160, tBackbitmap: (ref this.OwnBitmap), bbx: 535, bby: 65, theight: 30);
        this.opt3id = this.AddSubPart(ref tsubpart5, 535, 65, 160, 30, 1);
      }
      else
      {
        let mut tsubpart6: SubPartClass =  new TextButtonPartClass("Go to fill mode", 160, tBackbitmap: (ref this.OwnBitmap), bbx: 535, bby: 35, tinactive: true, theight: 30);
        this.opt4id = this.AddSubPart(ref tsubpart6, 535, 35, 160, 30, 1);
        let mut tsubpart7: SubPartClass =  new TextButtonPartClass("End drawing mode", 160, tBackbitmap: (ref this.OwnBitmap), bbx: 535, bby: 65, tinactive: true, theight: 30);
        this.opt5id = this.AddSubPart(ref tsubpart7, 535, 65, 160, 30, 1);
      }
    }

    pub WindowDescription: String(x: i32, y: i32)
    {
      if (this.game.SelectX < 0 || this.game.Data.Turn == -1)
        return "";
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
          return this.MouseText[index];
      }
      return "";
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num1: i32 =  this.SubPartID[index];
            if (num1 == this.MapListId)
            {
              let mut num2: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              if (num2 > -1)
              {
                this.detailnr = num2;
                this.game.EditObj.MapSelected = num2;
                this.game.CornerX = 0;
                this.game.CornerY = 0;
                this.game.SelectX = 0;
                this.game.SelectY = 0;
                this.game.EditObj.UnitSelected = -1;
                this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, 200, 150, false);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 20);
                windowReturnClass.AddCommand(4, 18);
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.pickid)
            {
              this.game.EditObj.PopupValue = 16;
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.AddCommand(5, 10);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.AddMapId)
            {
              str1: String = Conversions.ToString(Conversion.Val(Interaction.InputBox("Give width of map in hexes (10-200)", "Shadow Empire : Planetary Conquest", "20")));
              let mut w: i32 =  Operators.CompareString(Strings.Trim(str1), "", false) == 0 ? 0 : Conversions.ToInteger(str1);
              str2: String = Conversions.ToString(Conversion.Val(Interaction.InputBox("Give height of map in hexes (10-200)", "Shadow Empire : Planetary Conquest", "20")));
              let mut h: i32 =  Operators.CompareString(Strings.Trim(str2), "", false) == 0 ? 0 : Conversions.ToInteger(str2);
              if (w < 2 | h < 2 | w > 200 | h > 200)
              {
                let mut num3: i32 =   Interaction.MsgBox( "Cannot comply. Width and Height must be between 2 and 200", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                this.game.Data.AddMap(w, h);
                this.game.Data.MapObj[this.game.Data.MapCounter].Name = Interaction.InputBox("Give Name for Map", "Shadow Empire : Planetary Conquest");
                this.game.Data.MapObj[this.game.Data.MapCounter].MapLoop = false;
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else
            {
              if (num1 == this.maploopid)
              {
                if (this.detailnr > -1)
                {
                  if (Interaction.MsgBox( "You want to rename map?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                    this.game.Data.MapObj[this.detailnr].Name = Interaction.InputBox("Give Name for Map", "Shadow Empire : Planetary Conquest");
                  if (Interaction.MsgBox( "You want to change looping of the map?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                    this.game.Data.MapObj[this.detailnr].MapLoop = !this.game.Data.MapObj[this.detailnr].MapLoop;
                }
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.RemoveMapId)
              {
                if (this.detailnr > 0)
                {
                  this.game.Data.RemoveMap(this.detailnr);
                }
                else
                {
                  let mut num4: i32 =   Interaction.MsgBox( "Cannot delete. You must keep at least one map.");
                }
                this.game.EditObj.MapSelected = 0;
                this.game.SelectX = 0;
                this.game.SelectY = 0;
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 18);
                windowReturnClass.AddCommand(4, 20);
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.Shortsdesc)
              {
                this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.BBackId)
              {
                this.game.EditObj.InEditor = false;
                if (this.game.ModIntroType == 0)
                  windowReturnClass.AddCommand(3, 1);
                else
                  windowReturnClass.AddCommand(3, 12);
              }
              else if (num1 == this.BLTId)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 11);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BHisId)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 86);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BRoadId)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 14);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BLibId)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 93);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BConnectionID)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 58);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BRegimeID)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 15);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.bStringListId)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 60);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BShowMapId)
              {
                this.dostuff();
                if (this.game.EditObj.MiddleWindow)
                {
                  windowReturnClass.AddCommand(1, 4);
                  windowReturnClass.AddCommand(2, 12);
                  windowReturnClass.AddCommand(2, 18);
                  windowReturnClass.AddCommand(2, 20);
                  windowReturnClass.AddCommand(2, 44);
                  this.game.EditObj.MiddleWindow = false;
                }
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BSFTypeID)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 16);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BRiverID)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 33);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BActionID)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 57);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BBridgeID)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 34);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BPeopleID)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 22);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BEditUnitID)
              {
                if (this.game.SelectX > -1 & this.game.SelectY > -1)
                {
                  windowReturnClass.AddCommand(1, 4);
                  windowReturnClass.AddCommand(2, 17);
                  windowReturnClass.AddCommand(1, 2);
                  windowReturnClass.AddCommand(1, 5);
                  windowReturnClass.AddCommand(1, 7);
                  this.game.EditObj.MiddleWindow = true;
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num5: i32 =   Interaction.MsgBox( "No Hex has been selected. Order is cancelled.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
              }
              else if (num1 == this.BDefaultStringsID)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 23);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.bLocTypId)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 21);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BItemTypeID)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 24);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BLocationID)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 26);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BResID)
              {
                windowReturnClass.AddCommand(1, 4);
                windowReturnClass.AddCommand(2, 37);
                windowReturnClass.AddCommand(1, 2);
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(1, 7);
                this.game.EditObj.MiddleWindow = true;
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.BEventTypeID)
              {
                windowReturnClass.AddCommand(3, 7);
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.opt1id)
              {
                this.game.EditObj.PencilMode = 0;
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.opt2id)
              {
                this.game.EditObj.PencilMode = 1;
                this.dostuff();
                windowReturnClass.SetFlag(true);
              }
              else if (num1 == this.opt3id)
              {
                this.game.EditObj.PencilType = 0;
                this.game.EditObj.PencilMode = 0;
                this.game.EditObj.PaintShortcut1 = -1;
                this.game.EditObj.PaintShortcut2 = -1;
                this.game.EditObj.PaintShortcut3 = -1;
                this.dostuff();
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.SetFlag(true);
              }
            }
            return windowReturnClass;
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
