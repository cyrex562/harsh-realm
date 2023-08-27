// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.NewUnitWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class NewUnitWindowClass : WindowClass
  {
     B1Id: i32;
     B1TextId: i32;
     B2Id: i32;
     B2TextId: i32;
     B2bId: i32;
     B2bTextId: i32;
     B3Id: i32;
     B3TextId: i32;
     B4Id: i32;
     B4TextId: i32;
     B5Id: i32;
     B5TextId: i32;
     B6Id: i32;
     B6TextId: i32;
     off1id: i32;
     detailnr: i32;
     Text1Id: i32;
     Text2Id: i32;
     Text3Id: i32;
     Pic1Id: i32;
     detailnr2: i32;
     OrderTextId: i32;
     OrderText2Id: i32;
     OrderUpId: i32;
     OrderDownId: i32;
     ExtraId: i32;
     steppy: i32;
     typpy: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     OptionsList2Id: i32;
     ListClass OptionsList2Obj;
     bool Hq;

    pub NewUnitWindowClass( tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 =  -1, let mut sy: i32 =  -1)
      : base( tGame, 1024, 200, 99, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      this.steppy = 0;
      this.typpy = -1;
      this.detailnr = -1;
      this.detailnr2 = -1;
      this.fixshade = true;
      if (this.game.Data.Round == 0 & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].Regime > -1)
        this.game.Data.Turn = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].Regime;
      this.dostuff();
    }

     void dostuff()
    {
      if (this.off1id > 0)
        this.RemoveSubPart(this.off1id);
      if (this.Text1Id > 0)
        this.RemoveSubPart(this.Text1Id);
      if (this.Text2Id > 0)
        this.RemoveSubPart(this.Text2Id);
      if (this.Text3Id > 0)
        this.RemoveSubPart(this.Text3Id);
      if (this.Pic1Id > 0)
        this.RemoveSubPart(this.Pic1Id);
      if (this.B1Id > 0)
        this.RemoveSubPart(this.B1Id);
      if (this.B1TextId > 0)
        this.RemoveSubPart(this.B1TextId);
      if (this.B2Id > 0)
        this.RemoveSubPart(this.B2Id);
      if (this.B2TextId > 0)
        this.RemoveSubPart(this.B2TextId);
      if (this.B2bId > 0)
        this.RemoveSubPart(this.B2bId);
      if (this.B2bTextId > 0)
        this.RemoveSubPart(this.B2bTextId);
      if (this.B3Id > 0)
        this.RemoveSubPart(this.B3Id);
      if (this.B3TextId > 0)
        this.RemoveSubPart(this.B3TextId);
      if (this.B4Id > 0)
        this.RemoveSubPart(this.B4Id);
      if (this.B4TextId > 0)
        this.RemoveSubPart(this.B4TextId);
      if (this.B5Id > 0)
        this.RemoveSubPart(this.B5Id);
      if (this.B5TextId > 0)
        this.RemoveSubPart(this.B5TextId);
      if (this.B6Id > 0)
        this.RemoveSubPart(this.B6Id);
      if (this.B6TextId > 0)
        this.RemoveSubPart(this.B6TextId);
      if (this.OrderUpId > 0)
        this.RemoveSubPart(this.OrderUpId);
      if (this.OrderDownId > 0)
        this.RemoveSubPart(this.OrderDownId);
      if (this.ExtraId > 0)
        this.RemoveSubPart(this.ExtraId);
      if (this.OrderTextId > 0)
        this.RemoveSubPart(this.OrderTextId);
      if (this.OrderText2Id > 0)
        this.RemoveSubPart(this.OrderText2Id);
      if (this.OptionsList2Id > 0)
      {
        this.RemoveSubPart(this.OptionsList2Id);
        this.OptionsList2Id = 0;
      }
      this.NewBackGroundAndClearAll(1024, 200, -1);
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      let mut tsubpart1: SubPartClass =  new ATTextPartClass("Create what type of unit?", this.game.VicFont1, 400, 24, false);
      this.ExtraId = this.AddSubPart( tsubpart1, 410, 22, 400, 24, 0);
      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].UnitCounter < 15)
      {
        if ( this.game.Data.RuleVar[526] == 0.0)
        {
          if ( this.game.Data.RegimeObj[this.game.Data.Turn].ResPts >=  this.game.Data.RuleVar[46])
          {
            let mut tsubpart2: SubPartClass =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 440, bby: 60);
            this.B1Id = this.AddSubPart( tsubpart2, 440, 60, 32, 32, 1);
          }
          let mut tsubpart3: SubPartClass =  new ATTextPartClass("Formation (" + Strings.Trim(Conversion.Str( this.game.Data.RuleVar[46])) + " pp)", this.game.VicFont2, 400, 24, false);
          this.B1TextId = this.AddSubPart( tsubpart3, 480, 69, 400, 24, 0);
          if ( this.game.Data.RuleVar[343] > 0.0)
          {
            if ( this.game.Data.RegimeObj[this.game.Data.Turn].ResPts >=  this.game.Data.RuleVar[47] +  this.game.Data.RuleVar[345])
            {
              let mut tsubpart4: SubPartClass =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 440, bby: 100);
              this.B2bId = this.AddSubPart( tsubpart4, 440, 100, 32, 32, 1);
            }
            let mut tsubpart5: SubPartClass =  new ATTextPartClass("HQ (" + Strings.Trim(Conversion.Str( this.game.Data.RuleVar[47])) + " pp)", this.game.VicFont2, 400, 24, false);
            this.B2bTextId = this.AddSubPart( tsubpart5, 480, 109, 400, 24, 0);
            if ( this.game.Data.RuleVar[345] > 0.0 & this.game.Data.RegimeObj[this.game.Data.Turn].OfficerPool > -1 & this.game.Data.Round > 0)
            {
              if ( this.game.Data.RegimeObj[this.game.Data.Turn].ResPts >=  this.game.Data.RuleVar[47] +  this.game.Data.RuleVar[345])
              {
                let mut tsubpart6: SubPartClass =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 440, bby: 140);
                this.B2Id = this.AddSubPart( tsubpart6, 440, 140, 32, 32, 1);
              }
              let mut tsubpart7: SubPartClass =  new ATTextPartClass("HQ + Leader (" + Strings.Trim(Conversion.Str(  ( this.game.Data.RuleVar[47] +  this.game.Data.RuleVar[345]))) + " pp)", this.game.VicFont2, 400, 24, false);
              this.B2TextId = this.AddSubPart( tsubpart7, 480, 149, 400, 24, 0);
            }
          }
          else
          {
            if ( this.game.Data.RegimeObj[this.game.Data.Turn].ResPts >=  this.game.Data.RuleVar[47])
            {
              let mut tsubpart8: SubPartClass =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 440, bby: 100);
              this.B2Id = this.AddSubPart( tsubpart8, 440, 100, 32, 32, 1);
            }
            let mut tsubpart9: SubPartClass =  new ATTextPartClass("HQ (" + Strings.Trim(Conversion.Str( this.game.Data.RuleVar[47])) + " pp)", this.game.VicFont2, 400, 24, false);
            this.B2TextId = this.AddSubPart( tsubpart9, 480, 109, 400, 24, 0);
          }
        }
        else
        {
          this.OptionsListObj = ListClass::new();
          let mut num: i32 =  -1;
          let mut tlistselect1: i32 =  -1;
          let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
          for (let mut tdata: i32 =  0; tdata <= historicalUnitCounter; tdata += 1)
          {
            if (this.game.Data.HistoricalUnitObj[tdata].Model & this.game.Data.HistoricalUnitObj[tdata].TempRegime == this.game.Data.Turn)
            {
              num += 1;
              if (tdata == this.detailnr)
                tlistselect1 = num;
              this.OptionsListObj.add(this.game.Data.HistoricalUnitObj[tdata].Name, tdata, Conversion.Str( this.game.Data.HistoricalUnitObj[tdata].PP));
            }
          }
          if (this.OptionsListId > 0)
          {
            this.SubPartList[this.SubpartNr(this.OptionsListId)].Refresh(this.OptionsListObj, tlistselect1);
            this.SubPartFlag[this.SubpartNr(this.OptionsListId)] = true;
          }
          else
          {
            ListClass optionsListObj = this.OptionsListObj;
            let mut tlistselect2: i32 =  tlistselect1;
            let mut game: GameClass = this.game;
             local1: Bitmap =  this.OwnBitmap;
            font: Font =  null;
             local2: Font =  font;
            let mut tsubpart10: SubPartClass =  new ListSubPartClass(optionsListObj, 9, 500, tlistselect2, game, tShowPair: true, tValueWidth: 100, tdotopandbottom: false, tbackbitmap: ( local1), bbx: 310, bby: 55, overruleFont: ( local2));
            this.OptionsListId = this.AddSubPart( tsubpart10, 10, 30, 500, 160, 0);
          }
          if (this.detailnr > -1)
          {
            if (this.game.Data.RegimeObj[this.game.Data.Turn].ResPts >= this.game.Data.HistoricalUnitObj[this.detailnr].PP & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter + this.game.HandyFunctionsObj.GetHistoricalsSubUnitCount(this.detailnr) < 16)
            {
              let mut tsubpart11: SubPartClass =  ButtonPartClass::new(this.game.OKBALL);
              this.B3Id = this.AddSubPart( tsubpart11, 530, 30, 32, 32, 1);
            }
            else
            {
              let mut tsubpart12: SubPartClass =  ButtonPartClass::new(this.game.OKBALL, 1);
              this.B4Id = this.AddSubPart( tsubpart12, 530, 30, 32, 32, 0);
            }
            let mut tsubpart13: SubPartClass =  TextPartClass::new("Make new unit", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 24, false);
            this.B3TextId = this.AddSubPart( tsubpart13, 580, 39, 200, 24, 0);
          }
        }
      }
      else
      {
        let mut tsubpart14: SubPartClass =  new ATTextPartClass("To many units on hex to create a new one.", this.game.VicFont2, 400, 24, false);
        this.B1TextId = this.AddSubPart( tsubpart14, 50, 89, 400, 24, 0);
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      OrderResult orderResult1 = OrderResult::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 =  this.SubPartID[index1];
            if (num1 == this.B3Id)
            {
              this.game.ProcessingObj.AddNewUnitBasedOnHistorical(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.Data.Turn, this.detailnr);
              this.game.EditObj.OrderType = 3;
              this.game.EditObj.OrderUnit = this.game.Data.UnitCounter;
              this.game.EditObj.UnitSelected = this.game.Data.UnitCounter;
              windowReturnClass.AddCommand(1, 5);
              windowReturnClass.AddCommand(4, 18);
              windowReturnClass.AddCommand(2, 20);
              this.game.EditObj.TempCoordList = CoordList::new();
              this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 44);
              windowReturnClass.AddCommand(4, 66);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B1Id)
            {
              OrderResult orderResult2 = this.game.ProcessingObj.NewUnit(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.MapSelected, false, this.game.Data.Turn);
              if (this.game.EditObj.SoundOn)
                SoundMod.PlayAWave(this.game.AppPath + "sound/radio.wav",  this.game.EditObj);
              if (orderResult2.OK)
              {
                let mut num2: i32 =  0;
                let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                for (let mut index2: i32 =  0; index2 <= unitCounter; index2 += 1)
                {
                  if (this.game.Data.UnitObj[index2].IsHQ & this.game.Data.UnitObj[index2].PreDef == -1 & this.game.Data.UnitObj[index2].X > -1 && this.game.Data.UnitObj[index2].Regime == this.game.Data.Turn)
                    num2 = 1;
                }
                if (num2 == 1)
                {
                  this.game.EditObj.OrderType = 3;
                  this.game.EditObj.OrderUnit = this.game.Data.UnitCounter;
                  this.game.EditObj.UnitSelected = this.game.Data.UnitCounter;
                  windowReturnClass.AddCommand(1, 5);
                  windowReturnClass.AddCommand(4, 18);
                  windowReturnClass.AddCommand(2, 20);
                  this.game.EditObj.TempCoordList = CoordList::new();
                  this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 66);
                  windowReturnClass.AddCommand(4, 44);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                this.game.EditObj.OrderType = 0;
                this.game.EditObj.UnitSelected = this.game.Data.UnitCounter;
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(4, 18);
                windowReturnClass.AddCommand(2, 20);
                this.game.EditObj.TempCoordList = CoordList::new();
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 66);
                windowReturnClass.AddCommand(4, 44);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else if (num1 == this.B2Id)
            {
              OrderResult orderResult3 = this.game.ProcessingObj.NewUnit(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.MapSelected, true, this.game.Data.Turn);
              if (this.game.EditObj.SoundOn)
                SoundMod.PlayAWave(this.game.AppPath + "sound/radio.wav",  this.game.EditObj);
              if (orderResult3.OK)
              {
                this.game.EditObj.OrderType = 3;
                this.game.EditObj.OrderUnit = this.game.Data.UnitCounter;
                this.game.EditObj.UnitSelected = this.game.Data.UnitCounter;
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(2, 20);
                windowReturnClass.AddCommand(4, 18);
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 44);
                windowReturnClass.AddCommand(4, 66);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else if (num1 == this.B2bId)
            {
              OrderResult orderResult4 = this.game.ProcessingObj.NewUnit(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.MapSelected, true, this.game.Data.Turn, true);
              if (this.game.EditObj.SoundOn)
                SoundMod.PlayAWave(this.game.AppPath + "sound/radio.wav",  this.game.EditObj);
              if (orderResult4.OK)
              {
                this.game.EditObj.OrderType = 3;
                this.game.EditObj.OrderUnit = this.game.Data.UnitCounter;
                this.game.EditObj.UnitSelected = this.game.Data.UnitCounter;
                windowReturnClass.AddCommand(1, 5);
                windowReturnClass.AddCommand(2, 20);
                windowReturnClass.AddCommand(4, 18);
                this.game.EditObj.TempCoordList = CoordList::new();
                this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 44);
                windowReturnClass.AddCommand(4, 66);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else
            {
              if (num1 == this.OptionsListId)
              {
                let mut num3: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                this.SubPartFlag[index1] = true;
                if (num3 > -1)
                {
                  this.detailnr = num3;
                  this.dostuff();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.OptionsList2Id)
              {
                let mut num4: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                this.SubPartFlag[index1] = true;
                if (num4 > -1)
                {
                  this.detailnr2 = num4;
                  this.dostuff();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
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
