// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.NewUnit2WindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class NewUnit2WindowClass : WindowClass
  {
     B1Id: i32;
     B1TextId: i32;
     B2Id: i32;
     B2TextId: i32;
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
     int[] Ucnt;
     int[] ModCnt;
     ModSubCnt: Vec<i32>;
     SubCnt: Vec<i32>;

    pub NewUnit2WindowClass( tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 =  -1, let mut sy: i32 =  -1)
      : base( tGame, 1024, 200, 99, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      this.Ucnt = new int[1];
      this.ModCnt = new int[1];
      this.ModSubCnt = new int[1, 1];
      this.SubCnt = new int[1, 1];
      this.steppy = 0;
      this.typpy = -1;
      this.detailnr = -1;
      this.detailnr2 = -1;
      this.calc();
      this.dostuff();
    }

     void calc()
    {
      this.Ucnt = new int[this.game.Data.HistoricalUnitCounter + 1];
      this.ModCnt = new int[this.game.Data.HistoricalUnitCounter + 1];
      this.SubCnt = new int[this.game.Data.HistoricalUnitCounter + 1, 10];
      this.ModSubCnt = new int[this.game.Data.HistoricalUnitCounter + 1, 10];
      let mut unitCounter: i32 =  this.game.Data.UnitCounter;
      for (let mut index1: i32 =  0; index1 <= unitCounter; index1 += 1)
      {
        if (this.game.Data.UnitObj[index1].Historical > -1 & this.game.Data.UnitObj[index1].PreDef == -1 & this.game.Data.UnitObj[index1].Regime == this.game.Data.Turn)
        {
          int[] ucnt = this.Ucnt;
          int[] numArray1 = ucnt;
          let mut historical1: i32 =  this.game.Data.UnitObj[index1].Historical;
          let mut index2: i32 =  historical1;
          let mut num1: i32 =  ucnt[historical1] + 1;
          numArray1[index2] = num1;
          if (this.game.Data.UnitObj[index1].HistoricalSubPart > -1)
          {
            subCnt: Vec<i32> = this.SubCnt;
            numArray2: Vec<i32> = subCnt;
            let mut historical2: i32 =  this.game.Data.UnitObj[index1].Historical;
            let mut index3: i32 =  historical2;
            let mut historicalSubPart: i32 =  this.game.Data.UnitObj[index1].HistoricalSubPart;
            let mut index4: i32 =  historicalSubPart;
            let mut num2: i32 =  subCnt[historical2, historicalSubPart] + 1;
            numArray2[index3, index4] = num2;
          }
        }
      }
      let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
      for (let mut index5: i32 =  0; index5 <= historicalUnitCounter; index5 += 1)
      {
        let mut index6: i32 =  0;
        do
        {
          if (this.game.Data.HistoricalUnitObj[index5].SubParts[index6] > -1)
          {
            int[] modCnt = this.ModCnt;
            int[] numArray = modCnt;
            let mut index7: i32 =  index5;
            let mut index8: i32 =  index7;
            let mut num: i32 =  modCnt[index7] + 1;
            numArray[index8] = num;
          }
          modSubCnt: Vec<i32> = this.ModSubCnt;
          numArray3: Vec<i32> = modSubCnt;
          let mut index9: i32 =  index5;
          let mut index10: i32 =  index9;
          let mut index11: i32 =  index6;
          let mut index12: i32 =  index11;
          let mut num3: i32 =  modSubCnt[index9, index11] + 1;
          numArray3[index10, index12] = num3;
          index6 += 1;
        }
        while (index6 <= 9);
      }
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
      this.NewBackGroundAndClearAll(1024, 200, -1);
      Graphics objgraphics = Graphics.FromImage((Image) this.OwnBitmap);
      let mut tsubpart1: SubPartClass =  TextPartClass::new("Sub Unit Options (add, remove, change)", Font::new("Times New Roman", 22f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 24, false);
      this.ExtraId = this.AddSubPart( tsubpart1, 10, 2, 400, 24, 0);
      SubPartClass tsubpart2;
      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].UnitCounter < 15)
      {
        this.OptionsListObj = ListClass::new();
        let mut num1: i32 =  -1;
        let mut tlistselect1: i32 =  -1;
        let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
        for (let mut tdata: i32 =  0; tdata <= historicalUnitCounter; tdata += 1)
        {
          if (this.game.Data.HistoricalUnitObj[tdata].TempRegime == this.game.Data.Turn & this.game.Data.HistoricalUnitObj[tdata].ModelMaster > -1 && this.Ucnt[tdata] < this.ModCnt[tdata] & this.Ucnt[tdata] > 0)
          {
            num1 += 1;
            if (tdata == this.detailnr)
              tlistselect1 = num1;
            this.OptionsListObj.add(this.game.Data.HistoricalUnitObj[tdata].Name, tdata);
          }
        }
        if (num1 == -1)
        {
          tsubpart2 =  TextPartClass::new("No units that are missing subunits.", Font::new("Times New Roman", 14f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 24, false);
          this.B1TextId = this.AddSubPart( tsubpart2, 50, 89, 400, 24, 0);
          if (this.OptionsListId > 0)
          {
            this.RemoveSubPart(this.OptionsListId);
            this.OptionsListId = 0;
          }
          if (this.OptionsList2Id > 0)
          {
            this.RemoveSubPart(this.OptionsList2Id);
            this.OptionsList2Id = 0;
          }
          this.detailnr = -1;
          this.detailnr2 = -1;
          if (this.B3Id > 0)
            this.RemoveSubPart(this.B3Id);
          if (this.B4Id > 0)
            this.RemoveSubPart(this.B4Id);
        }
        else if (this.OptionsListId > 0)
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
          let mut tsubpart3: SubPartClass =  new ListSubPartClass(optionsListObj, 9, 350, tlistselect2, game, tShowPair: true, tValueWidth: 100, tdotopandbottom: false, tbackbitmap: ( local1), bbx: 10, bby: 30, overruleFont: ( local2));
          this.OptionsListId = this.AddSubPart( tsubpart3, 10, 30, 350, 160, 0);
        }
        if (this.detailnr > -1)
        {
          this.OptionsList2Obj = ListClass::new();
          let mut num2: i32 =  -1;
          let mut tlistselect3: i32 =  -1;
          let mut tdata: i32 =  0;
          do
          {
            if (this.game.Data.HistoricalUnitObj[this.detailnr].SubParts[tdata] > -1 && this.SubCnt[this.detailnr, tdata] == 0 & this.ModSubCnt[this.detailnr, tdata] > 0)
            {
              num2 += 1;
              if (tdata == this.detailnr2)
                tlistselect3 = num2;
              this.OptionsList2Obj.add(this.game.Data.UnitObj[this.game.HandyFunctionsObj.GetPreDef(this.game.Data.HistoricalUnitObj[this.detailnr].SubParts[tdata])].Name, tdata, Conversion.Str(  Math.Round( this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitObj[this.detailnr].ModelMaster].PP /  this.ModCnt[this.detailnr])));
            }
            tdata += 1;
          }
          while (tdata <= 9);
          if (this.OptionsList2Id > 0)
          {
            this.SubPartList[this.SubpartNr(this.OptionsList2Id)].Refresh(this.OptionsList2Obj, tlistselect3);
            this.SubPartFlag[this.SubpartNr(this.OptionsList2Id)] = true;
          }
          else
          {
            ListClass optionsList2Obj = this.OptionsList2Obj;
            let mut tlistselect4: i32 =  tlistselect3;
            let mut game: GameClass = this.game;
             local3: Bitmap =  this.OwnBitmap;
            font: Font =  null;
             local4: Font =  font;
            tsubpart2 =  new ListSubPartClass(optionsList2Obj, 9, 300, tlistselect4, game, tShowPair: true, tValueWidth: 100, tdotopandbottom: false, tbackbitmap: ( local3), bbx: 400, bby: 30, overruleFont: ( local4));
            this.OptionsList2Id = this.AddSubPart( tsubpart2, 400, 30, 300, 160, 0);
          }
          if (this.detailnr2 > -1)
          {
            if (this.game.Data.RegimeObj[this.game.Data.Turn].ResPts >=  Math.Round( this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitObj[this.detailnr].ModelMaster].PP /  this.ModCnt[this.detailnr]) & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter < 15)
            {
              tsubpart2 =  ButtonPartClass::new(this.game.OKBALL);
              this.B3Id = this.AddSubPart( tsubpart2, 750, 30, 32, 32, 1);
            }
            else
            {
              tsubpart2 =  ButtonPartClass::new(this.game.OKBALL, 1);
              this.B4Id = this.AddSubPart( tsubpart2, 750, 30, 32, 32, 0);
            }
            tsubpart2 =  TextPartClass::new("Make new unit", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 24, false);
            this.B3TextId = this.AddSubPart( tsubpart2, 800, 39, 200, 24, 0);
            if (this.game.EditObj.OrderUnit > -1 && !this.game.Data.UnitObj[this.game.EditObj.OrderUnit].IsHQ)
            {
               let mut local5: &Graphics = &objgraphics;
              bitmap: Bitmap = this.game.CustomBitmapObj.DrawUnit(this.game.EditObj.OrderUnit);
               let mut local6: &Bitmap = &bitmap;
              DrawMod.DrawSimple( local5,  local6, 750, 75);
              DrawMod.DrawText( objgraphics, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Name, this.game.GameFont1, 793, 73);
              tsubpart2 =  ButtonPartClass::new(this.game.OKBALL);
              this.B5Id = this.AddSubPart( tsubpart2, 795, 90, 32, 32, 1);
              tsubpart2 =  TextPartClass::new("Set selected unit", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 24, false);
              this.B5TextId = this.AddSubPart( tsubpart2, 835, 100, 200, 24, 0);
            }
          }
        }
      }
      else
      {
        tsubpart2 =  TextPartClass::new("To many units on hex to create a new one.", Font::new("Times New Roman", 14f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 24, false);
        this.B1TextId = this.AddSubPart( tsubpart2, 50, 89, 400, 24, 0);
      }
      if (this.game.EditObj.OrderUnit > -1 && !this.game.Data.UnitObj[this.game.EditObj.OrderUnit].IsHQ && this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical > -1)
      {
         let mut local7: &Graphics = &objgraphics;
        bitmap: Bitmap = this.game.CustomBitmapObj.DrawUnit(this.game.EditObj.OrderUnit);
         let mut local8: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local7,  local8, 750, 145);
        DrawMod.DrawText( objgraphics, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Name, this.game.GameFont1, 793, 143);
        tsubpart2 =  ButtonPartClass::new(this.game.CANCELBALL);
        this.B6Id = this.AddSubPart( tsubpart2, 795, 160, 32, 32, 1);
        tsubpart2 =  TextPartClass::new("Unassign Subunit", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 24, false);
        this.B6TextId = this.AddSubPart( tsubpart2, 835, 170, 200, 24, 0);
      }
      if (Information.IsNothing( objgraphics))
        return;
      objgraphics.Dispose();
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      OrderResult orderResult = OrderResult::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num1: i32 =  this.SubPartID[index];
            if (num1 == this.B3Id)
            {
              this.game.ProcessingObj.AddNewUnitBasedOnHistorical(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.Data.Turn, this.detailnr, this.detailnr2);
              windowReturnClass.AddCommand(4, 18);
              this.game.EditObj.TempCoordList = CoordList::new();
              this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 44);
              this.calc();
              this.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B5Id)
            {
              this.game.ProcessingObj.AddNewUnitBasedOnHistorical(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.Data.Turn, this.detailnr, this.detailnr2, this.game.EditObj.OrderUnit);
              this.game.EditObj.OrderType = 0;
              this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
              windowReturnClass.AddCommand(4, 18);
              this.game.EditObj.TempCoordList = CoordList::new();
              this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
              windowReturnClass.AddCommand(1, 5);
              windowReturnClass.AddCommand(4, 44);
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(2, 20);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B6Id)
            {
              if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical > -1 && this.game.Data.UnitObj[this.game.EditObj.OrderUnit].HistoricalSubPart > -1)
                this.game.Data.UnitObj[this.game.EditObj.OrderUnit].StartPower = this.game.HandyFunctionsObj.GetPowerPtsAbsolute(this.game.HandyFunctionsObj.GetPreDef(this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical].SubParts[this.game.Data.UnitObj[this.game.EditObj.OrderUnit].HistoricalSubPart]), true);
              this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical = -1;
              this.game.Data.UnitObj[this.game.EditObj.OrderUnit].HistoricalSubPart = -1;
              this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Name = this.game.Data.RegimeObj[this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Regime].UnitName;
              this.game.EditObj.OrderType = 0;
              this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
              windowReturnClass.AddCommand(4, 18);
              this.game.EditObj.TempCoordList = CoordList::new();
              this.game.EditObj.TempCoordList.AddCoord(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap);
              windowReturnClass.AddCommand(1, 5);
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 44);
              windowReturnClass.AddCommand(2, 20);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsListId)
            {
              let mut num2: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (num2 > -1)
              {
                this.detailnr = num2;
                this.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsList2Id)
            {
              let mut num3: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (num3 > -1)
              {
                this.detailnr2 = num3;
                this.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
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
