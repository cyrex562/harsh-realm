// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.EditorPaintWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Text;

namespace WindowsApplication1
{
  pub class EditorPaintWindowClass : WindowClass
  {
     TempText1: i32;
     temptext2: i32;
     temptext3: i32;
     temptext4: i32;
     temptext5: i32;
     temptext6: i32;
     temptext7: i32;
     temptext8: i32;
     temptext9: i32;
     temptext10: i32;
     TempText11: i32;
     temptext12: i32;
     temptext13: i32;
     temptext14: i32;
     temptext15: i32;
     temptext16: i32;
     temptext17: i32;
     temptext18: i32;
     temptext19: i32;
     temptext20: i32;
     TempText21: i32;
     temptext22: i32;
     temptext23: i32;
     temptext24: i32;
     temptext25: i32;
     temptext26: i32;
     temptext27: i32;
     temptext28: i32;
     temptext29: i32;
     temptext30: i32;
     TempText31: i32;
     temptext32: i32;
     temptext33: i32;
     temptext34: i32;
     temptext35: i32;
     temptext36: i32;
     temptext37: i32;
     temptext38: i32;
     temptext39: i32;
     temptext40: i32;
     temptext41: i32;
     temptext42: i32;
     temptext43: i32;
     temptext44: i32;
     temptext45: i32;
     temptext46: i32;
     LogoListId: i32;
     but1id: i32;
     tab1id: i32;
     tab2id: i32;
     tab3id: i32;
     tab4id: i32;
     tab5id: i32;
     but1textid: i32;
     but1bid: i32;
     hqbut0: i32;
     hqbut1: i32;
     hqbut2: i32;
     but2id: i32;
     but2textid: i32;
     but3id: i32;
     but3textid: i32;
     but4id: i32;
     but4textid: i32;
     but5id: i32;
     but5textid: i32;
     but6id: i32;
     but6textid: i32;
     but7id: i32;
     quitid: i32;
     but7textid: i32;
     descid: i32;
     comparenr: i32;
     sliderid: i32;
     logolist2id: i32;
     logolist3id: i32;
     float tempBlink;
     unr: i32;
     sfnr: i32;
     sftyp: i32;
     detailnr: i32;
     detailnr2: i32;
     detailtype: i32;
     ammount: i32;
     bool hqreach;
     passenger: i32;
     firstlistId: i32;
     ATListClass firstlistObj;
     medlistId: i32;
     ATListClass medListObj;
     lastListId: i32;
     ATListClass lastListObj;
     OptionsList4Id: i32;
     ATListClass OptionsList4Obj;
     OptionsList5Id: i32;
     ATListClass OptionsList5Obj;
     OptionsList6Id: i32;
     ATListClass OptionsList6Obj;
     combatListId: i32;
     ATListClass combatListObj;
     combatList2Id: i32;
     ATListClass combatList2Obj;
     StatTyp: i32;
     StatMode: i32;
     int[] ChainHq;
     HQselect: i32;
     infoid: i32;
     ltnr: i32;
     locnr: i32;
     ppl: i32;
     spnr: i32;

    pub handleTimer: WindowReturnClass() => WindowReturnClass::new();

    pub EditorPaintWindowClass(ref tGame: GameClass)
      : base(ref tGame, 1024, 768, 9, tDoBorders: 1, tHeaderString: "Select patype: i32")
    {
      this.ChainHq = new int[3];
      this.tempBlink = 0.0f;
      this.game.EditObj.CurrentDescript = "";
      this.unr = -1;
      this.ppl = this.game.EditObj.PeopleSelected;
      this.StatMode = 0;
      this.detailnr = -1;
      this.detailnr2 = 0;
      this.DoStuff();
    }

    pub fn DoStuff()
    {
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      graphics.SmoothingMode = SmoothingMode.AntiAlias;
      graphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      graphics.TextContrast = 1;
      this.NewBackGroundAndClearAll(1024, 768, -1);
      this.BackBitmap = (Bitmap) this.OwnBitmap.Clone();
      if (this.but1id > 0)
        this.RemoveSubPart(this.but1id);
      if (this.firstlistId > 0)
        this.RemoveSubPart(this.firstlistId);
      if (this.medlistId > 0)
        this.RemoveSubPart(this.medlistId);
      if (this.lastListId > 0)
        this.RemoveSubPart(this.lastListId);
      if (this.TempText1 > 0)
        this.RemoveSubPart(this.TempText1);
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("OK", 320, tBackbitmap: (ref this.OwnBitmap), bbx: 312, bby: 680, theight: 45);
      this.but1id = this.AddSubPart(ref tsubpart1, 312, 680, 320, 45, 1);
      this.firstlistObj = ATListClass::new();
      let mut tlistselect1: i32 =  -1;
      if (this.game.EditObj.inSimpleEditor & !this.game.EditObj.inSimpleMapEditor)
      {
        this.firstlistObj.add("Regimes", 3);
        if (this.game.EditObj.PaintShortcut1 == 3)
          tlistselect1 = 0;
        this.firstlistObj.add("Libvar Slots", 11);
        if (this.game.EditObj.PaintShortcut1 == 11)
          tlistselect1 = 1;
      }
      else if (this.game.EditObj.inSimpleMapEditor)
      {
        this.firstlistObj.add("Landscapes", 1);
        if (this.game.EditObj.PaintShortcut1 == 1)
          tlistselect1 = 0;
        this.firstlistObj.add("Roads", 2);
        if (this.game.EditObj.PaintShortcut1 == 2)
          tlistselect1 = 1;
        this.firstlistObj.add("Location Types", 4);
        if (this.game.EditObj.PaintShortcut1 == 4)
          tlistselect1 = 2;
        this.firstlistObj.add("Rivers", 5);
        if (this.game.EditObj.PaintShortcut1 == 5)
          tlistselect1 = 3;
        this.firstlistObj.add("Bridge", 6);
        if (this.game.EditObj.PaintShortcut1 == 6)
          tlistselect1 = 4;
        this.firstlistObj.add("Areaslots", 9);
        if (this.game.EditObj.PaintShortcut1 == 9)
          tlistselect1 = 5;
        if (this.game.AllowHeightMap)
        {
          this.firstlistObj.add("Height Map", 12);
          if (this.game.EditObj.PaintShortcut1 == 12)
            tlistselect1 = 6;
        }
      }
      else
      {
        this.firstlistObj.add("Landscapes", 1);
        if (this.game.EditObj.PaintShortcut1 == 1)
          tlistselect1 = 0;
        this.firstlistObj.add("Roads", 2);
        if (this.game.EditObj.PaintShortcut1 == 2)
          tlistselect1 = 1;
        this.firstlistObj.add("Regimes", 3);
        if (this.game.EditObj.PaintShortcut1 == 3)
          tlistselect1 = 2;
        this.firstlistObj.add("LocType", 4);
        if (this.game.EditObj.PaintShortcut1 == 4)
          tlistselect1 = 3;
        this.firstlistObj.add("Rivers", 5);
        if (this.game.EditObj.PaintShortcut1 == 5)
          tlistselect1 = 4;
        this.firstlistObj.add("Bridge", 6);
        if (this.game.EditObj.PaintShortcut1 == 6)
          tlistselect1 = 5;
        this.firstlistObj.add("Slots", 9);
        if (this.game.EditObj.PaintShortcut1 == 9)
          tlistselect1 = 6;
        this.firstlistObj.add("Specials", 10);
        if (this.game.EditObj.PaintShortcut1 == 10)
          tlistselect1 = 7;
        this.firstlistObj.add("Libvar Slots", 11);
        if (this.game.EditObj.PaintShortcut1 == 11)
          tlistselect1 = 8;
        if (this.game.AllowHeightMap)
        {
          this.firstlistObj.add("Height Map", 12);
          if (this.game.EditObj.PaintShortcut1 == 12)
            tlistselect1 = 9;
        }
      }
      let mut tsubpart2: SubPartClass =  new ATListSubPartClass(this.firstlistObj, 18, 250, tlistselect1, this.game, tHeader: "What?", tbackbitmap: (ref this.OwnBitmap), bbx: 50, bby: 50);
      this.firstlistId = this.AddSubPart(ref tsubpart2, 50, 50, 250, 336, 0);
      str1: String = "";
      if (this.game.EditObj.PaintShortcut1 == 1)
      {
        this.medListObj = ATListClass::new();
        str1 = "Landscapes".to_owned();
        let mut num: i32 =  -1;
        let mut landscapeTypeCounter: i32 =  this.game.Data.LandscapeTypeCounter;
        for (let mut tdata: i32 =  0; tdata <= landscapeTypeCounter; tdata += 1)
        {
          if (!this.game.Data.LandscapeTypeObj[tdata].DontShowInList)
          {
            this.medListObj.add(this.game.Data.LandscapeTypeObj[tdata].Name, tdata);
            num += 1;
            if (tdata == this.game.EditObj.PaintShortcut2)
              tlistselect1 = num;
          }
        }
      }
      else if (this.game.EditObj.PaintShortcut1 == 2)
      {
        this.medListObj = ATListClass::new();
        str1 = "Roads".to_owned();
        let mut roadTypeCounter: i32 =  this.game.Data.RoadTypeCounter;
        for (let mut tdata: i32 =  0; tdata <= roadTypeCounter; tdata += 1)
          this.medListObj.add(this.game.Data.RoadTypeObj[tdata].Name, tdata);
      }
      else if (this.game.EditObj.PaintShortcut1 == 3)
      {
        this.medListObj = ATListClass::new();
        str1 = "Regimes".to_owned();
        let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
        for (let mut tdata: i32 =  0; tdata <= regimeCounter; tdata += 1)
          this.medListObj.add(this.game.Data.RegimeObj[tdata].Name, tdata);
        this.medListObj.add("Neutral", this.game.Data.RegimeCounter + 1);
      }
      else if (this.game.EditObj.PaintShortcut1 == 4)
      {
        this.medListObj = ATListClass::new();
        str1 = "LocType".to_owned();
        let mut num: i32 =  -1;
        let mut locTypeCounter: i32 =  this.game.Data.LocTypeCounter;
        for (let mut tdata: i32 =  0; tdata <= locTypeCounter; tdata += 1)
        {
          if (!this.game.Data.LocTypeObj[tdata].editorBlock)
          {
            num += 1;
            this.medListObj.add(this.game.Data.LocTypeObj[tdata].Name, tdata);
            if (tdata == this.game.EditObj.PaintShortcut2)
              tlistselect1 = num;
          }
        }
      }
      else if (this.game.EditObj.PaintShortcut1 == 5)
      {
        this.medListObj = ATListClass::new();
        str1 = "Rivers".to_owned();
        let mut riverTypeCounter: i32 =  this.game.Data.RiverTypeCounter;
        for (let mut tdata: i32 =  0; tdata <= riverTypeCounter; tdata += 1)
          this.medListObj.add(this.game.Data.RiverTypeObj[tdata].Name, tdata);
      }
      else if (this.game.EditObj.PaintShortcut1 == 6)
      {
        this.medListObj = ATListClass::new();
        str1 = "Bridges".to_owned();
        this.medListObj.add("Bridge", 0);
      }
      else if (this.game.EditObj.PaintShortcut1 == 9)
      {
        this.medListObj = ATListClass::new();
        str1 = "Slot Numbers (non-library)";
        let mut num: i32 =  0;
        do
        {
          this.medListObj.add(Strings.Trim(Conversion.Str( num)) + ") " + this.game.Data.TempString[710 + num], num);
          num += 1;
        }
        while (num <= 9);
      }
      else if (this.game.EditObj.PaintShortcut1 == 10)
      {
        this.medListObj = ATListClass::new();
        str1 = "Landscapes".to_owned();
        this.medListObj.add("No Special", -1);
        let mut landscapeTypeCounter: i32 =  this.game.Data.LandscapeTypeCounter;
        for (let mut tdata: i32 =  0; tdata <= landscapeTypeCounter; tdata += 1)
          this.medListObj.add(this.game.Data.LandscapeTypeObj[tdata].Name, tdata);
      }
      else if (this.game.EditObj.PaintShortcut1 == 11)
      {
        this.medListObj = ATListClass::new();
        str1 = "LibVar slots";
        let mut num: i32 =  -1;
        let mut libVarCounter: i32 =  this.game.Data.LibVarCounter;
        for (let mut tdata: i32 =  0; tdata <= libVarCounter; tdata += 1)
        {
          if (this.game.Data.LibVarObj[tdata].type == NewEnums.LibVarType.Hex)
          {
            num += 1;
            this.medListObj.add(this.game.Data.LibVarObj[tdata].name, tdata);
            if (tdata == this.game.EditObj.PaintShortcut2)
              tlistselect1 = num;
          }
        }
      }
      else if (this.game.EditObj.PaintShortcut1 == 12)
      {
        this.medListObj = ATListClass::new();
        str1 = "Height Map";
        this.medListObj.add("Height Map Values", 0);
        this.medListObj.add("Left-Right Click", 1);
      }
      if (this.game.EditObj.PaintShortcut1 != 1 && this.game.EditObj.PaintShortcut1 != 4 && this.game.EditObj.PaintShortcut1 != 11)
        tlistselect1 = this.game.EditObj.PaintShortcut2;
      if (Strings.Len(str1) > 0)
      {
        if (this.game.EditObj.PaintShortcut1 == 1 | this.game.EditObj.PaintShortcut1 >= 9)
        {
          tsubpart2 =  new ATListSubPartClass(this.medListObj, 36, 250, tlistselect1, this.game, tHeader: str1, tbackbitmap: (ref this.OwnBitmap), bbx: 350, bby: 50);
          this.medlistId = this.AddSubPart(ref tsubpart2, 350, 50, 250, 624, 0);
        }
        else
        {
          tsubpart2 =  new ATListSubPartClass(this.medListObj, 36, 250, tlistselect1, this.game, tHeader: str1, tbackbitmap: (ref this.OwnBitmap), bbx: 350, bby: 50);
          this.medlistId = this.AddSubPart(ref tsubpart2, 350, 50, 250, 624, 0);
        }
      }
      str2: String = "";
      tlistselect2: i32;
      if ((this.game.EditObj.PaintShortcut1 == 1 | this.game.EditObj.PaintShortcut1 == 10) & this.game.EditObj.PaintShortcut2 > -1)
      {
        this.lastListObj = ATListClass::new();
        str2 = "Sprites".to_owned();
        let mut basicSpriteCounter: i32 =  this.game.Data.LandscapeTypeObj[this.game.EditObj.PaintShortcut2].BasicSpriteCounter;
        for (let mut tdata: i32 =  0; tdata <= basicSpriteCounter; tdata += 1)
          this.lastListObj.add(this.game.Data.LandscapeTypeObj[this.game.EditObj.PaintShortcut2].BasicSpriteFileName[tdata], tdata);
      }
      else if (this.game.EditObj.PaintShortcut1 == 9 & this.game.EditObj.PaintShortcut2 > -1)
      {
        this.lastListObj = ATListClass::new();
        str2 = "Values".to_owned();
        let mut num: i32 =  0;
        do
        {
          let mut areaBySlot: i32 =  this.game.HandyFunctionsObj.GetAreaBySlot(this.game.EditObj.PaintShortcut2, num);
          if (areaBySlot > -1)
            this.lastListObj.add(Strings.Trim(Conversion.Str( num)) + ") " + this.game.Data.AreaObj[areaBySlot].Name, num);
          else
            this.lastListObj.add(Strings.Trim(Conversion.Str( num)), num);
          num += 1;
        }
        while (num <= 125);
      }
      else if (this.game.EditObj.PaintShortcut1 == 12 & this.game.EditObj.PaintShortcut2 == 0)
      {
        this.lastListObj = ATListClass::new();
        str2 = "Values".to_owned();
        let mut num1: i32 =  -1;
        tlistselect2 = -1;
        let mut num2: i32 =  0;
        do
        {
          num1 += 1;
          if (num2 == this.game.EditObj.PaintShortcut3)
            tlistselect2 = num1;
          this.lastListObj.add("Height Level " + Strings.Trim(Conversion.Str( num2)), num2);
          num2 += 1;
        }
        while (num2 <= 9);
      }
      else if (this.game.EditObj.PaintShortcut1 == 11 & this.game.EditObj.PaintShortcut2 > -1)
      {
        this.lastListObj = ATListClass::new();
        str2 = "Values".to_owned();
        let mut num3: i32 =  -1;
        tlistselect2 = -1;
        let mut index: i32 =  -1;
        let mut num4: i32 =  0;
        do
        {
          num3 += 1;
          if (num4 == this.game.EditObj.PaintShortcut3)
            tlistselect2 = num3;
          if (index > -1)
            this.lastListObj.add(Strings.Trim(Conversion.Str( num4)) + ") " + this.game.Data.AreaObj[index].Name, num4);
          else
            this.lastListObj.add(Strings.Trim(Conversion.Str( num4)), num4);
          num4 += 1;
        }
        while (num4 <= 100);
        let mut num5: i32 =  105;
        do
        {
          num3 += 1;
          if (num5 == this.game.EditObj.PaintShortcut3)
            tlistselect2 = num3;
          if (index > -1)
            this.lastListObj.add(Strings.Trim(Conversion.Str( num5)) + ") " + this.game.Data.AreaObj[index].Name, num5);
          else
            this.lastListObj.add(Strings.Trim(Conversion.Str( num5)), num5);
          num5 += 5;
        }
        while (num5 <= 250);
        let mut num6: i32 =  260;
        do
        {
          num3 += 1;
          if (num6 == this.game.EditObj.PaintShortcut3)
            tlistselect2 = num3;
          if (index > -1)
            this.lastListObj.add(Strings.Trim(Conversion.Str( num6)) + ") " + this.game.Data.AreaObj[index].Name, num6);
          else
            this.lastListObj.add(Strings.Trim(Conversion.Str( num6)), num6);
          num6 += 10;
        }
        while (num6 <= 500);
        let mut num7: i32 =  550;
        do
        {
          num3 += 1;
          if (num7 == this.game.EditObj.PaintShortcut3)
            tlistselect2 = num3;
          if (index > -1)
            this.lastListObj.add(Strings.Trim(Conversion.Str( num7)) + ") " + this.game.Data.AreaObj[index].Name, num7);
          else
            this.lastListObj.add(Strings.Trim(Conversion.Str( num7)), num7);
          num7 += 50;
        }
        while (num7 <= 2000);
        let mut num8: i32 =  2500;
        do
        {
          num3 += 1;
          if (num8 == this.game.EditObj.PaintShortcut3)
            tlistselect2 = num3;
          if (index > -1)
            this.lastListObj.add(Strings.Trim(Conversion.Str( num8)) + ") " + this.game.Data.AreaObj[index].Name, num8);
          else
            this.lastListObj.add(Strings.Trim(Conversion.Str( num8)), num8);
          num8 += 500;
        }
        while (num8 <= 10000);
      }
      if (!(this.game.EditObj.PaintShortcut1 == 11 & this.game.EditObj.PaintShortcut2 > -1))
        tlistselect2 = this.game.EditObj.PaintShortcut3;
      if (Strings.Len(str2) <= 0)
        return;
      tsubpart2 =  new ATListSubPartClass(this.lastListObj, 36, 250, tlistselect2, this.game, tHeader: str2, tbackbitmap: (ref this.OwnBitmap), bbx: 650, bby: 50);
      this.lastListId = this.AddSubPart(ref tsubpart2, 650, 50, 250, 624, 0);
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          this.game.EditObj.TempCoordList = CoordList::new();
          windowReturnClass.AddCommand(6, 0);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
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
            if (num1 == this.but1id)
            {
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.firstlistId)
            {
              let mut num2: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (num2 > -1)
              {
                this.game.EditObj.PencilType = num2;
                this.game.EditObj.PaintShortcut1 = num2;
                this.game.EditObj.PaintShortcut2 = -1;
                this.game.EditObj.PaintShortcut3 = -1;
                if (num2 == 1 | num2 == 9)
                {
                  this.game.EditObj.PaintShortcut2 = 0;
                  this.game.EditObj.PaintShortcut3 = 0;
                  this.game.EditObj.PencilData1 = 0;
                  this.game.EditObj.PencilData2 = 0;
                }
                if (num2 == 2 | num2 == 4 | num2 == 5 | num2 == 6)
                {
                  this.game.EditObj.PaintShortcut2 = 0;
                  this.game.EditObj.PencilData1 = 0;
                }
                this.DoStuff();
                windowReturnClass.SetFlag(true);
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.medlistId)
            {
              let mut num3: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              let mut num4: i32 =  -1;
              if (this.SubpartNr(this.lastListId) > 0)
                num4 = this.SubPartList[this.SubpartNr(this.lastListId)].GetSelect();
              this.SubPartFlag[index] = true;
              if (num3 > -1)
              {
                this.game.EditObj.PaintShortcut2 = num3;
                this.game.EditObj.PaintShortcut3 = num4;
                if (this.game.EditObj.PaintShortcut1 == 2)
                {
                  this.game.EditObj.PencilType = 2;
                  this.game.EditObj.PencilData1 = this.game.EditObj.PaintShortcut2;
                }
                else if (this.game.EditObj.PaintShortcut1 == 3)
                {
                  this.game.EditObj.PencilType = 3;
                  this.game.EditObj.PencilData1 = this.game.EditObj.PaintShortcut2;
                }
                else if (this.game.EditObj.PaintShortcut1 == 4)
                {
                  this.game.EditObj.PencilType = 4;
                  this.game.EditObj.PencilData1 = this.game.EditObj.PaintShortcut2;
                }
                else if (this.game.EditObj.PaintShortcut1 == 5)
                {
                  this.game.EditObj.PencilType = 5;
                  this.game.EditObj.PencilData1 = this.game.EditObj.PaintShortcut2;
                }
                else if (this.game.EditObj.PaintShortcut1 == 6)
                {
                  this.game.EditObj.PencilType = 6;
                  this.game.EditObj.PencilData1 = 0;
                }
                else if (this.game.EditObj.PaintShortcut1 == 1 | this.game.EditObj.PaintShortcut1 == 10)
                {
                  this.game.EditObj.PencilType = this.game.EditObj.PaintShortcut1;
                  this.game.EditObj.PencilData1 = this.game.EditObj.PaintShortcut2;
                  this.game.EditObj.PencilData2 = this.game.EditObj.PaintShortcut3;
                }
                else if (this.game.EditObj.PaintShortcut1 == 9)
                {
                  this.game.EditObj.PencilType = 9;
                  this.game.EditObj.PencilData1 = this.game.EditObj.PaintShortcut2;
                  this.game.EditObj.PencilData2 = this.game.EditObj.PaintShortcut3;
                }
                else if (this.game.EditObj.PaintShortcut1 == 11)
                {
                  this.game.EditObj.PencilType = 11;
                  this.game.EditObj.PencilData1 = this.game.EditObj.PaintShortcut2;
                  this.game.EditObj.PencilData2 = this.game.EditObj.PaintShortcut3;
                }
                else if (this.game.EditObj.PaintShortcut1 == 12)
                {
                  this.game.EditObj.PencilType = 12;
                  this.game.EditObj.PencilData1 = this.game.EditObj.PaintShortcut2;
                  this.game.EditObj.PencilData2 = this.game.EditObj.PaintShortcut3;
                }
                this.DoStuff();
                windowReturnClass.SetFlag(true);
              }
              else if (this.game.EditObj.PaintShortcut1 == 10)
              {
                this.game.EditObj.PencilType = this.game.EditObj.PaintShortcut1;
                this.game.EditObj.PencilData1 = -1;
                this.game.EditObj.PencilData2 = -1;
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 != this.lastListId)
              return windowReturnClass;
            let mut num5: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
            this.SubPartFlag[index] = true;
            if (num5 > -1)
            {
              this.game.EditObj.PaintShortcut3 = num5;
              if (this.game.EditObj.PaintShortcut1 == 1 | this.game.EditObj.PaintShortcut1 == 10)
              {
                this.game.EditObj.PencilType = this.game.EditObj.PaintShortcut1;
                this.game.EditObj.PencilData1 = this.game.EditObj.PaintShortcut2;
                this.game.EditObj.PencilData2 = this.game.EditObj.PaintShortcut3;
              }
              else if (this.game.EditObj.PaintShortcut1 == 9)
              {
                this.game.EditObj.PencilType = 9;
                this.game.EditObj.PencilData1 = this.game.EditObj.PaintShortcut2;
                this.game.EditObj.PencilData2 = this.game.EditObj.PaintShortcut3;
              }
              else if (this.game.EditObj.PaintShortcut1 == 11)
              {
                this.game.EditObj.PencilType = 11;
                this.game.EditObj.PencilData1 = this.game.EditObj.PaintShortcut2;
                this.game.EditObj.PencilData2 = this.game.EditObj.PaintShortcut3;
              }
              else if (this.game.EditObj.PaintShortcut1 == 12)
              {
                this.game.EditObj.PencilType = 12;
                this.game.EditObj.PencilData1 = this.game.EditObj.PaintShortcut2;
                this.game.EditObj.PencilData2 = this.game.EditObj.PaintShortcut3;
              }
              this.DoStuff();
              windowReturnClass.SetFlag(true);
            }
            windowReturnClass.SetFlag(true);
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
