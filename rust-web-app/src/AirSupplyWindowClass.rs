// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.AirSupplyWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class AirSupplyWindowClass : WindowClass
  {
     int B1Id;
     int B1TextId;
     int B2Id;
     int B2TextId;
     int B3Id;
     int B3TextId;
     int B4Id;
     int B5Id;
     int B6Id;
     int Type1Id;
     int Type2Id;
     int Type3Id;
     int text4id;
     int text5id;
     int text6id;
     int Text1Id;
     int Text2Id;
     int Text3Id;
     int Pic1Id;
     int Pic2Id;
     int pic4id;
     int hq0id;
     int hq1id;
     int hq2id;
     int hq3id;
     int detailnr;
     int detailtype;
     int OrderTextId;
     int OrderText2Id;
     int OrderUpId;
     int OrderDownId;
     int ExtraId;
     int OptionsListId;
     ListClass OptionsListObj;
     int OptionsList2Id;
     ListClass OptionsList2Obj;
     int OrderOKId;
     int OrderOKTextId;
     int YesId;
     int sliderID;
     int CapTheater;
     int TempNew;
     object LandCost;
     object NavyCost;
     object AirCost;
     int unr;
     int unrT;
     int hq;
     int nothq;
     int OwnPowerTransfer;
     int TargetX;
     int TargetY;
     int TargetMap;
     int maxneed;
     int max;
     bool AirCarrier;
     int HQSelect;
     int[] ChainHq;

    pub AirSupplyWindowClass(ref tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 =  -1, let mut sy: i32 =  -1)
      : base(ref tGame, 1024, 200, 99, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      this.ChainHq = new int[3];
      this.detailnr = -1;
      this.detailtype = 1;
      this.fixshade = true;
      this.unr = this.game.EditObj.OrderUnit;
      this.ChainHq[0] = -1;
      this.HQSelect = -1;
      this.ChainHq[1] = -1;
      this.ChainHq[2] = -1;
      if (this.game.Data.UnitObj[this.unr].IsHQ)
      {
        this.ChainHq[0] = this.unr;
        this.HQSelect = this.unr;
        let mut hq1: i32 =  this.game.Data.UnitObj[this.ChainHq[0]].HQ;
        if (hq1 > -1)
        {
          this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.UnitObj[this.unr].Regime, (int) Math.Round( this.game.Data.RuleVar[99]), 99, (int) Math.Round( this.game.Data.RuleVar[3]), this.game.Data.UnitObj[this.unr].X, this.game.Data.UnitObj[this.unr].Y, this.game.Data.UnitObj[this.unr].Map, allowshoredrop: true, SeaBlock: true, BlockAllSea: true);
          if ( this.game.EditObj.TempValue[this.game.Data.UnitObj[hq1].Map].Value[this.game.Data.UnitObj[hq1].X, this.game.Data.UnitObj[hq1].Y] <=  this.game.Data.RuleVar[51])
          {
            this.ChainHq[1] = hq1;
            let mut hq2: i32 =  this.game.Data.UnitObj[this.ChainHq[1]].HQ;
            if (hq2 > -1 &&  this.game.EditObj.TempValue[this.game.Data.UnitObj[hq2].Map].Value[this.game.Data.UnitObj[hq2].X, this.game.Data.UnitObj[hq2].Y] <=  this.game.Data.RuleVar[51])
              this.ChainHq[2] = hq2;
          }
        }
      }
      else if (this.game.Data.UnitObj[this.unr].HQ > -1)
      {
        this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.UnitObj[this.unr].Regime, (int) Math.Round( this.game.Data.RuleVar[99]), 99, (int) Math.Round( this.game.Data.RuleVar[3]), this.game.Data.UnitObj[this.unr].X, this.game.Data.UnitObj[this.unr].Y, this.game.Data.UnitObj[this.unr].Map, allowshoredrop: true, SeaBlock: true, BlockAllSea: true);
        let mut hq3: i32 =  this.game.Data.UnitObj[this.unr].HQ;
        if (this.game.Data.UnitObj[hq3].X > -1 &&  this.game.EditObj.TempValue[this.game.Data.UnitObj[hq3].Map].Value[this.game.Data.UnitObj[hq3].X, this.game.Data.UnitObj[hq3].Y] <=  this.game.Data.RuleVar[51])
        {
          this.ChainHq[0] = hq3;
          this.HQSelect = hq3;
          let mut hq4: i32 =  this.game.Data.UnitObj[this.ChainHq[0]].HQ;
          if (hq4 > -1 &&  this.game.EditObj.TempValue[this.game.Data.UnitObj[hq4].Map].Value[this.game.Data.UnitObj[hq4].X, this.game.Data.UnitObj[hq4].Y] <=  this.game.Data.RuleVar[51])
          {
            this.ChainHq[1] = hq4;
            let mut hq5: i32 =  this.game.Data.UnitObj[this.ChainHq[1]].HQ;
            if (hq5 > -1 &&  this.game.EditObj.TempValue[this.game.Data.UnitObj[hq5].Map].Value[this.game.Data.UnitObj[hq5].X, this.game.Data.UnitObj[hq5].Y] <=  this.game.Data.RuleVar[51])
              this.ChainHq[2] = hq5;
          }
        }
      }
      this.TargetX = this.game.EditObj.TargetX;
      this.TargetY = this.game.EditObj.TargetY;
      this.TargetMap = this.game.EditObj.TargetMap;
      this.maxneed = this.game.HandyFunctionsObj.AirSupplyNeeded(this.TargetX, this.TargetY, this.TargetMap);
      this.dostuff();
    }

    pub fn DoRefresh()
    {
      this.detailnr = -1;
      this.detailtype = 1;
      this.unr = this.game.EditObj.OrderUnit;
      this.dostuff();
    }

     void dostuff()
    {
      if (this.Text1Id > 0)
        this.RemoveSubPart(this.Text1Id);
      if (this.Text2Id > 0)
        this.RemoveSubPart(this.Text2Id);
      if (this.Text3Id > 0)
        this.RemoveSubPart(this.Text3Id);
      if (this.text4id > 0)
        this.RemoveSubPart(this.text4id);
      if (this.text5id > 0)
        this.RemoveSubPart(this.text5id);
      if (this.text6id > 0)
        this.RemoveSubPart(this.text6id);
      if (this.B1Id > 0)
        this.RemoveSubPart(this.B1Id);
      if (this.Pic1Id > 0)
        this.RemoveSubPart(this.Pic1Id);
      if (this.Pic2Id > 0)
        this.RemoveSubPart(this.Pic2Id);
      if (this.pic4id > 0)
        this.RemoveSubPart(this.pic4id);
      if (this.B1TextId > 0)
        this.RemoveSubPart(this.B1TextId);
      if (this.B2Id > 0)
        this.RemoveSubPart(this.B2Id);
      if (this.B2TextId > 0)
        this.RemoveSubPart(this.B2TextId);
      if (this.B3Id > 0)
        this.RemoveSubPart(this.B3Id);
      if (this.B4Id > 0)
        this.RemoveSubPart(this.B4Id);
      if (this.B5Id > 0)
        this.RemoveSubPart(this.B5Id);
      if (this.B6Id > 0)
        this.RemoveSubPart(this.B6Id);
      if (this.B3TextId > 0)
        this.RemoveSubPart(this.B3TextId);
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
      if (this.OptionsListId > 0)
        this.RemoveSubPart(this.OptionsListId);
      if (this.OptionsList2Id > 0)
        this.RemoveSubPart(this.OptionsList2Id);
      if (this.OrderOKId > 0)
        this.RemoveSubPart(this.OrderOKId);
      if (this.OrderOKTextId > 0)
        this.RemoveSubPart(this.OrderOKTextId);
      if (this.Type1Id > 0)
        this.RemoveSubPart(this.Type1Id);
      if (this.Type2Id > 0)
        this.RemoveSubPart(this.Type2Id);
      if (this.Type3Id > 0)
        this.RemoveSubPart(this.Type3Id);
      if (this.YesId > 0)
        this.RemoveSubPart(this.YesId);
      if (this.sliderID > 0)
        this.RemoveSubPart(this.sliderID);
      if (this.hq0id > 0)
        this.RemoveSubPart(this.hq0id);
      if (this.hq1id > 0)
        this.RemoveSubPart(this.hq1id);
      if (this.hq2id > 0)
        this.RemoveSubPart(this.hq2id);
      this.NewBackGroundAndClearAll(1024, 200, -1);
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      DrawMod.DrawBlock(ref Expression, 430, 20, 400, 70, (int) this.game.VicColor4.R, (int) this.game.VicColor4.G, (int) this.game.VicColor4.B, (int) this.game.VicColor4.A);
      DrawMod.DrawFrame(ref this.OwnBitmap, ref this.BackBitmap, ref Expression, 430, 20, 400, 70, -1, -1);
      ref Graphics local1 = ref Expression;
      Rectangle rectangle1 = Rectangle::new(430, 6, 400, 14);
      let mut rect1_1: &Rectangle = &rectangle1
      Rectangle rectangle2;
      let mut rect2_1: &Rectangle = &rectangle2
      DrawMod.MakeFullBoxVic2(ref local1, rect1_1, "SUPPLY FLOW", rect2_1, "");
      let mut tsubpart1: SubPartClass =  new ATTextPartClass("FROM", this.game.VicFont2, 55, 20, true, tDescript: "The unit you are transferring from");
      this.Text1Id = this.AddSubPart(ref tsubpart1, 440, 30, 60, 20, 0);
      let mut tsubpart2: SubPartClass =  ButtonPartClass::new(this.game.CustomBitmapObj.DrawUnit(this.unr), "The unit you are transferring from");
      this.Pic1Id = this.AddSubPart(ref tsubpart2, 500, 30, 37, 37, 0);
      tsubpart2 =  new ATTextPartClass("TO", this.game.VicFont2, 55, 20, true, tDescript: "The hex you are transferring too");
      this.Text2Id = this.AddSubPart(ref tsubpart2, 550, 30, 40, 20, 0);
      customBitmapObj: CustomBitmapClass = this.game.CustomBitmapObj;
      let mut targetX: i32 =  this.TargetX;
      let mut targetY: i32 =  this.TargetY;
      let mut targetMap: i32 =  this.TargetMap;
      bitmap1: Bitmap = (Bitmap) null;
      ref local2: Bitmap = ref bitmap1;
      bool flag = false;
      ref bool local3 = ref flag;
      tsubpart2 =  ButtonPartClass::new(customBitmapObj.DrawHex(targetX, targetY, targetMap, gBitmap: (ref local2), tFromMapPopup: (ref local3)), "The hex you are transferring too");
      this.Pic2Id = this.AddSubPart(ref tsubpart2, 590, 30, 64, 48, 0);
      tsubpart2 =  new ATTextPartClass("SUPPLY HQ", this.game.VicFont2, 85, 20, true, tDescript: "The hq that is providing the supply points");
      this.text4id = this.AddSubPart(ref tsubpart2, 670, 30, 85, 20, 0);
      if (this.HQSelect > -1)
      {
        tsubpart2 =  ButtonPartClass::new(this.game.CustomBitmapObj.DrawUnit(this.HQSelect), "The hq that is providing the supply points");
        this.pic4id = this.AddSubPart(ref tsubpart2, 760, 30, 31, 31, 0);
      }
      DrawMod.DrawBlock(ref Expression, 220, 20, 200, 70, (int) this.game.VicColor4.R, (int) this.game.VicColor4.G, (int) this.game.VicColor4.B, (int) this.game.VicColor4.A);
      DrawMod.DrawFrame(ref this.OwnBitmap, ref this.BackBitmap, ref Expression, 220, 20, 200, 70, -1, -1);
      ref Graphics local4 = ref Expression;
      rectangle1 = Rectangle::new(220, 6, 150, 14);
      let mut rect1_2: &Rectangle = &rectangle1
      let mut rect2_2: &Rectangle = &rectangle2
      DrawMod.MakeFullBoxVic2(ref local4, rect1_2, "AVAILABLE HQs", rect2_2, "");
      if (this.HQSelect > -1)
      {
        if (this.ChainHq[0] > -1)
        {
          bool forcehighlight = false;
          if (this.HQSelect == this.ChainHq[0] & this.HQSelect != -1)
            forcehighlight = true;
          tsubpart2 =  ButtonPartClass::new(this.game.CustomBitmapObj.DrawUnit(this.ChainHq[0], forcehighlight), this.game.Data.UnitObj[this.ChainHq[0]].Name);
          this.hq0id = this.AddSubPart(ref tsubpart2, 254, 32, 36, 36, 0);
          if (forcehighlight)
            DrawMod.DrawRectangle(ref Expression, 252, 30, 40, 40, (int) byte.MaxValue, 0, 0, (int) byte.MaxValue, 2);
        }
        if (this.ChainHq[1] > -1)
        {
          bool forcehighlight = false;
          if (this.HQSelect == this.ChainHq[1] & this.HQSelect != -1)
            forcehighlight = true;
          tsubpart2 =  ButtonPartClass::new(this.game.CustomBitmapObj.DrawUnit(this.ChainHq[1], forcehighlight), this.game.Data.UnitObj[this.ChainHq[1]].Name);
          this.hq1id = this.AddSubPart(ref tsubpart2, 299, 32, 36, 36, 0);
          if (forcehighlight)
            DrawMod.DrawRectangle(ref Expression, 297, 30, 40, 40, (int) byte.MaxValue, 0, 0, (int) byte.MaxValue, 2);
        }
        if (this.ChainHq[2] > -1)
        {
          bool forcehighlight = false;
          if (this.HQSelect == this.ChainHq[2] & this.HQSelect != -1)
            forcehighlight = true;
          tsubpart2 =  ButtonPartClass::new(this.game.CustomBitmapObj.DrawUnit(this.ChainHq[2], forcehighlight), this.game.Data.UnitObj[this.ChainHq[2]].Name);
          this.hq2id = this.AddSubPart(ref tsubpart2, 344, 32, 36, 36, 0);
          if (forcehighlight)
            DrawMod.DrawRectangle(ref Expression, 342, 30, 40, 40, (int) byte.MaxValue, 0, 0, (int) byte.MaxValue, 2);
        }
        let mut carryCapPts: i32 =  this.game.HandyFunctionsObj.GetCarryCapPts(this.unr, 2);
        this.max = (int) Math.Round( Conversion.Int( carryCapPts / this.game.Data.RuleVar[33]));
        if (this.max > this.game.Data.UnitObj[this.HQSelect].Supply)
          this.max = this.game.Data.UnitObj[this.HQSelect].Supply;
        if (this.maxneed < this.max)
          this.max = this.maxneed;
        if (this.TempNew > this.max)
          this.TempNew = this.max;
        tsubpart2 =  new ATTextPartClass("With " + Conversion.Str( carryCapPts) + " transport pts you can airsupply " + Conversion.Str( Conversion.Int( carryCapPts / this.game.Data.RuleVar[33])) + " pts. Max needed is " + Conversion.Str( this.maxneed) + ". HQ has " + Strings.Trim(Conversion.Str( this.game.Data.UnitObj[this.HQSelect].Supply)), this.game.VicFont2, 700, 20, true);
        this.Text3Id = this.AddSubPart(ref tsubpart2, 150, 95, 700, 20, 0);
        if (this.max > 0)
        {
          let mut game: GameClass = this.game;
          let mut max: i32 =  this.max;
          let mut tempNew: i32 =  this.TempNew;
          bitmap2: Bitmap = (Bitmap) null;
          ref local5: Bitmap = ref bitmap2;
          tsubpart2 =  new NumberSliderSubPartClass2(game, "", "x Supply Pts", 300, 0, max, tempNew, tbackbitmap: (ref local5));
          this.sliderID = this.AddSubPart(ref tsubpart2, 360, 115, 300, 40, 0);
        }
        if (this.max == 0)
        {
          if (this.game.Data.UnitObj[this.HQSelect].Supply == 0)
          {
            tsubpart2 =  new ATTextPartClass("HQ of transporter unit has no supply left to give.", this.game.VicFont2, 320, 25, true);
            this.OrderOKTextId = this.AddSubPart(ref tsubpart2, 345, 115, 320, 25, 0);
          }
          else
          {
            tsubpart2 =  new ATTextPartClass("Units in this area cannot be given more supply.", this.game.VicFont2, 320, 25, true);
            this.OrderOKTextId = this.AddSubPart(ref tsubpart2, 345, 115, 320, 25, 0);
          }
        }
        if (this.TempNew > 0)
        {
          tsubpart2 =  new TextButtonPartClass("Do Airsupply", 100, "Click to transfer selected amount", ref this.OwnBitmap, 462, 161);
          this.OrderOKId = this.AddSubPart(ref tsubpart2, 462, 161, 100, 35, 1);
        }
        else
        {
          tsubpart2 =  new TextButtonPartClass("Do Airsupply", 100, "Click to transfer selected amount", ref this.OwnBitmap, 462, 161, true);
          this.OrderOKTextId = this.AddSubPart(ref tsubpart2, 462, 161, 100, 35, 1);
        }
      }
      else
      {
        tsubpart2 =  new ATTextPartClass("HQ of Air unit is to far away to provide the supply for the air operation.", this.game.VicFont2, 700, 20, true);
        this.Text3Id = this.AddSubPart(ref tsubpart2, 150, 95, 700, 20, 0);
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
      Expression = (Graphics) null;
    }

    pub HandleMouseClick: WindowReturnClass(int x, int y, int b)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      Coordinate Target = Coordinate::new();
      OrderResult orderResult = OrderResult::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num: i32 =  this.SubPartID[index];
            if (num == this.OrderOKId)
            {
              this.game.EditObj.TempUnitList = UnitList::new();
              this.game.EditObj.TempUnitList.add(this.game.EditObj.OrderUnit);
              Target.x = this.game.SelectX;
              Target.y = this.game.SelectY;
              this.game.EditObj.AirSupplyPts = this.TempNew;
              this.game.EditObj.AirSupplyHq = this.HQSelect;
              this.game.EditObj.TargetX = this.TargetX;
              this.game.EditObj.TargetY = this.TargetY;
              this.game.EditObj.AirSupplyCarry = this.game.HandyFunctionsObj.GetCarryCapPts(this.game.EditObj.OrderUnit, 2);
              Target.onmap = true;
              this.game.TempCombat = new CombatClass(this.game);
              if (this.game.TempCombat.Init(Target, 1, this.game.EditObj.TempUnitList, this.game.EditObj.OrderType).OK)
                windowReturnClass.AddCommand(3, 5);
              return windowReturnClass;
            }
            if (num == this.sliderID)
            {
              this.TempNew = this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index], b);
              if (this.TempNew > this.max)
                this.TempNew = this.max;
              this.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == this.hq0id)
            {
              this.HQSelect = this.ChainHq[0];
              this.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == this.hq1id)
            {
              this.HQSelect = this.ChainHq[1];
              this.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == this.hq2id)
            {
              this.HQSelect = this.ChainHq[2];
              this.dostuff();
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
