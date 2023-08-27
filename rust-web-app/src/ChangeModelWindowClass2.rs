// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ChangeModelWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class ChangeModelWindowClass2 : WindowClass
  {
     int LocNr;
     int BNameId;
     int BNameTextId;
     int B1Id;
     int B1TextId;
     int B2Id;
     int B2TextId;
     int B3Id;
     int B3TextId;
     int Text1Id;
     int Text2Id;
     int Text3Id;
     int OptionsListId;
     ListClass OptionsListObj;
     int detailnr;
     int B4Id;
     int B4TextId;
     int B5Id;
     int B5TextId;
     int B6Id;
     int B6TextId;
     int off1id;
     int Pic1Id;
     int detailnr2;
     int OrderTextId;
     int OrderText2Id;
     int OrderUpId;
     int OrderDownId;
     int ExtraId;
     int steppy;
     int typpy;
     int OptionsList2Id;
     ListClass OptionsList2Obj;
     bool Hq;
     int w;
     int h;
     int[] Ucnt;
     int[] ModCnt;
     ModSubCnt: Vec<i32>;
     SubCnt: Vec<i32>;
     int[] modelcount;
     bool[] creatable;
     errors: Vec<String>;

    pub ChangeModelWindowClass2(ref tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 =  -1, let mut sy: i32 =  -1)
      : base(ref tGame, tGame.ScreenWidth, 222, BackSprite: tGame.MARCBOTBAR)
    {
      this.Ucnt = new int[1];
      this.ModCnt = new int[1];
      this.ModSubCnt = new int[1, 1];
      this.SubCnt = new int[1, 1];
      this.modelcount = new int[1];
      this.creatable = new bool[1];
      this.errors = new string[1];
      this.w = tGame.ScreenWidth;
      this.h = 222;
      this.BlockBlit = true;
      this.LocNr = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
      this.detailnr = -1;
      this.calc();
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
      let mut num1: i32 =   Math.Round( (this.w - 1024) / 2.0);
      this.NewBackGroundAndClearAll(this.w, this.h, this.game.MARCBOTBAR);
      this.ClearMouse();
      Graphics objgraphics = Graphics.FromImage((Image) this.OwnBitmap);
      let mut orderUnit: i32 =  this.game.EditObj.OrderUnit;
      tstring: String;
      int Number1;
      int hisnr;
      if (this.game.Data.UnitObj[orderUnit].Historical == -1)
      {
        tstring = "Unit is currently a to be disbanded formation.";
        Number1 = 1;
        hisnr = -1;
      }
      else if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[orderUnit].Historical].ModelMaster == -1)
      {
        tstring = "Unit is currently an ad hoc formation." + " Current unit consists of " + Conversion.Str( Number1) + " subunits.";
        hisnr = -1;
      }
      else
      {
        str: String = "Current Model is " + this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[orderUnit].Historical].ModelMaster].Name;
        let mut unitCounter: i32 =  this.game.Data.UnitCounter;
        int Number2;
        for (let mut index: i32 =  0; index <= unitCounter; index += 1)
        {
          if (this.game.Data.UnitObj[index].PreDef == -1 && this.game.Data.UnitObj[index].Historical == this.game.Data.UnitObj[orderUnit].Historical)
            Number2 += 1;
        }
        tstring = str + " Current unit consists of " + Conversion.Str( Number2) + " subunits.";
        hisnr = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[orderUnit].Historical].ModelMaster;
      }
      DrawMod.DrawTextColouredMarc(ref objgraphics, tstring, this.game.MarcFont4, num1 + 15, 10, Color.White);
      this.OptionsListObj = ListClass::new();
      let mut num2: i32 =  -1;
      let mut tlistselect1: i32 =  -1;
      let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
      for (let mut index: i32 =  0; index <= historicalUnitCounter; index += 1)
      {
        if (this.game.Data.HistoricalUnitObj[index].Model & !this.game.Data.HistoricalUnitObj[index].Fixed & this.game.Data.HistoricalUnitObj[index].TempRegime == this.game.Data.Turn && hisnr > -1 && this.game.HandyFunctionsObj.GetHistoricalsSubUnitCount(hisnr) <= this.game.HandyFunctionsObj.GetHistoricalsSubUnitCount(index) && this.game.Data.HistoricalUnitObj[index].Type < 5 && !(this.game.Data.HistoricalUnitObj[index].Type < 5 & this.game.Data.UnitObj[orderUnit].IsHQ) && this.game.Data.HistoricalUnitObj[index].MaxPresent != 0 && this.game.Data.HistoricalUnitObj[index].People == this.game.Data.HistoricalUnitObj[hisnr].People && hisnr != index)
        {
          num2 += 1;
          if (index == this.detailnr)
            tlistselect1 = num2;
          this.OptionsListObj.add(this.game.Data.HistoricalUnitObj[index].Name, index, Conversion.Str( this.game.HandyFunctionsObj.GetHistoricalsSubUnitCount(index)) + " units", this.game.Data.HistoricalUnitObj[index].PP.ToString() + "PP");
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
        ref local1: Bitmap = ref this.OwnBitmap;
        let mut bbx: i32 =  num1 + 10;
        font: Font =  null;
        ref local2: Font = ref font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(optionsListObj, 7, 500, tlistselect2, game, tShowPair: true, tValueWidth: 100, tdotopandbottom: false, tbackbitmap: (ref local1), bbx: bbx, bby: 50, tMarcStyle: true, overruleFont: (ref local2));
        this.OptionsListId = this.AddSubPart(ref tsubpart, num1 + 10, 50, 500, 128, 0);
      }
      if (this.detailnr > -1)
      {
        if (this.creatable[this.detailnr])
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("CHANGE TO THIS MODEL", 380, "Click to create to set the current unit to the selected model.", ref this.BackBitmap, num1 + 550, 130, theight: 50, usefont: this.game.MarcFont1, useshadow: true, tMarcStyle: true);
          this.B3Id = this.AddSubPart(ref tsubpart, num1 + 550, 130, 380, 50, 1);
        }
        else
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("CHANGE TO THIS MODEL", 380, this.errors[this.detailnr], ref this.BackBitmap, num1 + 550, 130, true, theight: 50, usefont: this.game.MarcFont1, useshadow: true, tMarcStyle: true);
          this.B3TextId = this.AddSubPart(ref tsubpart, num1 + 550, 130, 380, 50, 1);
        }
      }
      else
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("CHANGE TO THIS MODEL", 380, "Please select a unit model first.", ref this.BackBitmap, num1 + 550, 130, true, theight: 50, usefont: this.game.MarcFont1, useshadow: true, tMarcStyle: true);
        this.B3TextId = this.AddSubPart(ref tsubpart, num1 + 550, 130, 380, 50, 1);
      }
      if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical <= -1 || this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical].ModelMaster <= -1)
        ;
      if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical <= -1)
        ;
      if (Information.IsNothing( objgraphics))
        return;
      objgraphics.Dispose();
    }

     void calc()
    {
      this.Ucnt = new int[this.game.Data.HistoricalUnitCounter + 1];
      this.ModCnt = new int[this.game.Data.HistoricalUnitCounter + 1];
      this.SubCnt = new int[this.game.Data.HistoricalUnitCounter + 1, 10];
      this.ModSubCnt = new int[this.game.Data.HistoricalUnitCounter + 1, 10];
      this.modelcount = new int[this.game.Data.HistoricalUnitCounter + 1];
      this.creatable = new bool[this.game.Data.HistoricalUnitCounter + 1];
      this.errors = new string[this.game.Data.HistoricalUnitCounter + 1];
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
      let mut historicalUnitCounter1: i32 =  this.game.Data.HistoricalUnitCounter;
      for (let mut index5: i32 =  0; index5 <= historicalUnitCounter1; index5 += 1)
      {
        if (this.game.Data.HistoricalUnitObj[index5].ModelMaster > -1 && this.Ucnt[index5] > 0)
        {
          int[] modelcount = this.modelcount;
          int[] numArray = modelcount;
          let mut modelMaster: i32 =  this.game.Data.HistoricalUnitObj[index5].ModelMaster;
          let mut index6: i32 =  modelMaster;
          let mut num: i32 =  modelcount[modelMaster] + 1;
          numArray[index6] = num;
        }
      }
      let mut historicalUnitCounter2: i32 =  this.game.Data.HistoricalUnitCounter;
      for (let mut index: i32 =  0; index <= historicalUnitCounter2; index += 1)
      {
        this.errors[index] = "";
        if (this.game.Data.HistoricalUnitObj[index].Model)
        {
          this.creatable[index] = false;
          if (this.game.Data.RegimeObj[this.game.Data.Turn].ResPts < this.game.Data.HistoricalUnitObj[index].PP)
            this.errors[index] = "Not enough PP to create.";
          else if (this.modelcount[index] >= this.game.Data.HistoricalUnitObj[index].MaxPresent & this.game.Data.HistoricalUnitObj[index].MaxPresent != -1)
            this.errors[index] = "Maximum ammount of this model already on map.";
          else
            this.creatable[index] = true;
        }
      }
      let mut historicalUnitCounter3: i32 =  this.game.Data.HistoricalUnitCounter;
      for (let mut index7: i32 =  0; index7 <= historicalUnitCounter3; index7 += 1)
      {
        let mut index8: i32 =  0;
        do
        {
          if (this.game.Data.HistoricalUnitObj[index7].SubParts[index8] > -1)
          {
            int[] modCnt = this.ModCnt;
            int[] numArray = modCnt;
            let mut index9: i32 =  index7;
            let mut index10: i32 =  index9;
            let mut num: i32 =  modCnt[index9] + 1;
            numArray[index10] = num;
          }
          modSubCnt: Vec<i32> = this.ModSubCnt;
          numArray3: Vec<i32> = modSubCnt;
          let mut index11: i32 =  index7;
          let mut index12: i32 =  index11;
          let mut index13: i32 =  index8;
          let mut index14: i32 =  index13;
          let mut num3: i32 =  modSubCnt[index11, index13] + 1;
          numArray3[index12, index14] = num3;
          index8 += 1;
        }
        while (index8 <= 9);
      }
    }

    pub HandleMouseClick: WindowReturnClass(int x, int y, int b)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 =  this.SubPartID[index1];
            if (num1 == this.OptionsListId)
            {
              let mut num2: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                this.detailnr = num2;
                this.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B3Id)
            {
              this.game.ProcessingObj.AddNewUnitBasedOnHistorical(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, this.game.Data.Turn, this.detailnr, OverWriteUnr: this.game.EditObj.OrderUnit);
              this.game.EditObj.OrderType = 0;
              this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
              this.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(1, 5);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(2, 69);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 9);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B4Id)
            {
              this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical].ModelMaster = -1;
              this.game.EditObj.OrderType = 0;
              this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
              windowReturnClass.AddCommand(4, 18);
              this.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(1, 5);
              windowReturnClass.AddCommand(4, 44);
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(2, 20);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B5Id)
            {
              let mut unitCounter: i32 =  this.game.Data.UnitCounter;
              for (let mut index2: i32 =  0; index2 <= unitCounter; index2 += 1)
              {
                if (this.game.Data.UnitObj[index2].Historical == this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical & index2 != this.game.EditObj.OrderUnit)
                  this.game.Data.UnitObj[index2].Historical = -1;
              }
              this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical = -1;
              this.game.EditObj.OrderType = 0;
              this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
              windowReturnClass.AddCommand(4, 18);
              this.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(1, 5);
              windowReturnClass.AddCommand(4, 44);
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(2, 20);
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
