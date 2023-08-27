// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.OfficerPoolWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class OfficerPoolWindowClass : WindowClass
  {
     LocNr: i32;
     BNameId: i32;
     BNameTextId: i32;
     B1Id: i32;
     b1bid: i32;
     B1TextId: i32;
     B2Id: i32;
     b2bid: i32;
     B2TextId: i32;
     B3Id: i32;
     b3bid: i32;
     B3TextId: i32;
     B4Id: i32;
     b4bid: i32;
     Text1Id: i32;
     Text2Id: i32;
     Text3Id: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     detailnr: i32;

    pub OfficerPoolWindowClass( tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 =  -1, let mut sy: i32 =  -1)
      : base( tGame, 1024, 200, 99, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      this.LocNr = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
      this.detailnr = -1;
      if (this.game.Data.Round == 0 & this.game.EditObj.UnitSelected > -1)
        this.game.Data.Turn = this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime;
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
      if (this.B1Id > 0)
        this.RemoveSubPart(this.B1Id);
      if (this.B4Id > 0)
        this.RemoveSubPart(this.B4Id);
      if (this.b1bid > 0)
        this.RemoveSubPart(this.b1bid);
      if (this.b2bid > 0)
        this.RemoveSubPart(this.b2bid);
      if (this.b3bid > 0)
        this.RemoveSubPart(this.b3bid);
      if (this.b4bid > 0)
        this.RemoveSubPart(this.b4bid);
      if (this.B1TextId > 0)
        this.RemoveSubPart(this.B1TextId);
      if (this.B2Id > 0)
        this.RemoveSubPart(this.B2Id);
      if (this.B3Id > 0)
        this.RemoveSubPart(this.B3Id);
      if (this.B2TextId > 0)
        this.RemoveSubPart(this.B2TextId);
      this.NewBackGroundAndClearAll(1024, 200, -1);
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical > -1)
      {
        let mut tsubpart: SubPartClass =  new OfficerPartClass(this.game.EditObj.OrderUnit, this.game);
        this.Text1Id = this.AddSubPart( tsubpart, 0, 0, 300, 200, 0);
      }
      else
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("Selected HQ has no officer.", this.game.GameFont1, 250, 20, false);
        this.Text1Id = this.AddSubPart( tsubpart, 25, 50, 250, 20, 0);
      }
      if (this.OptionsListId == 0)
      {
        this.OptionsListObj = ListClass::new();
        let mut num: i32 =  -1;
        let mut tlistselect1: i32 =  -1;
        let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
        for (let mut tdata: i32 =  0; tdata <= historicalUnitCounter; tdata += 1)
        {
          if (this.game.Data.HistoricalUnitObj[tdata].Pool & this.game.Data.HistoricalUnitObj[tdata].TempRegime == this.game.Data.Turn)
          {
            this.OptionsListObj.add(this.game.Data.HistoricalUnitObj[tdata].CommanderName, tdata);
            num += 1;
            if (tdata == this.detailnr)
              tlistselect1 = num;
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
          let mut tsubpart: SubPartClass =  new ListSubPartClass(optionsListObj, 8, 270, tlistselect2, game, true, "Officer Pool", tbackbitmap: ( local1), bbx: 310, bby: 10, overruleFont: ( local2));
          this.OptionsListId = this.AddSubPart( tsubpart, 310, 10, 270, 176, 0);
        }
      }
      if (this.game.Data.Round == 0 | ((long) -( this.game.Data.RuleVar[345] <=  this.game.Data.RegimeObj[this.game.Data.Turn].ResPts ? 1 : 0) & (long) Math.Round( this.game.Data.RuleVar[345])) > 0L)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("Recruit (" + this.game.Data.RuleVar[345].ToString() + "pp)", 150, "Click to recruit an extra officer to the officerpool",  this.OwnBitmap, 600, 10);
        this.B1Id = this.AddSubPart( tsubpart, 600, 10, 150, 35, 1);
      }
      else if ( this.game.Data.RuleVar[345] > 0.0 & this.game.Data.RegimeObj[this.game.Data.Turn].OfficerPool > -1)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("Recruit", 150, "You do not have the PP to buy an officer ",  this.OwnBitmap, 600, 10, true);
        this.b1bid = this.AddSubPart( tsubpart, 600, 10, 150, 35, 0);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("Recruit", 150, "Recruiting is disabled. ",  this.OwnBitmap, 600, 10, true);
        this.b1bid = this.AddSubPart( tsubpart, 600, 10, 150, 35, 0);
      }
      int @this;
      if (this.detailnr > -1)
      {
        if (this.game.Data.Round == 0)
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("Swap", 150, "Click to swap officer in selected HQ with officer selected in officerpool",  this.OwnBitmap, 600, 50);
          this.B2Id = this.AddSubPart( tsubpart, 600, 50, 150, 35, 1);
        }
        else if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical > -1)
        {
          @this = Math.Abs(Math.Min(0, this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical].PP)) + Math.Max(this.game.Data.HistoricalUnitObj[this.detailnr].PP, 0);
          if (@this < 0)
            @this = 0;
          if (this.game.Data.RegimeObj[this.game.Data.Turn].ResPts >= this.game.Data.HistoricalUnitObj[this.detailnr].PP + Math.Min(this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical].PP, 0))
          {
            let mut tsubpart: SubPartClass =  new TextButtonPartClass("Swap (" + @this.ToString() + "pp)", 150, "Click to swap officer in selected HQ with officer selected in officerpool",  this.OwnBitmap, 600, 50);
            this.B2Id = this.AddSubPart( tsubpart, 600, 50, 150, 35, 1);
          }
          else
          {
            let mut tsubpart: SubPartClass =  new TextButtonPartClass("Swap (" + @this.ToString() + "pp)", 150, "You dont have the PP to appothis: i32 officer or the PP to remove the officer in the unit.",  this.OwnBitmap, 600, 50, true);
            this.b2bid = this.AddSubPart( tsubpart, 600, 50, 150, 35, 0);
          }
        }
        else
        {
          @this = Math.Max(this.game.Data.HistoricalUnitObj[this.detailnr].PP, 0);
          if (@this < 0)
            @this = 0;
          if (this.game.Data.RegimeObj[this.game.Data.Turn].ResPts >= @this)
          {
            let mut tsubpart: SubPartClass =  new TextButtonPartClass("Appoint (" + @this.ToString() + "pp)", 150, "Click to swap officer in selected HQ with officer selected in officerpool",  this.OwnBitmap, 600, 50);
            this.B2Id = this.AddSubPart( tsubpart, 600, 50, 150, 35, 1);
          }
          else
          {
            let mut tsubpart: SubPartClass =  new TextButtonPartClass("Appoint (" + @this.ToString() + "pp)", 150, "You dont have the PP to appothis: i32 officer or the PP to remove the officer in the unit.",  this.OwnBitmap, 600, 50, true);
            this.b2bid = this.AddSubPart( tsubpart, 600, 50, 150, 35, 0);
          }
        }
        let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Remove >", 150, "Click to remove the selected officer from the officerpool",  this.OwnBitmap, 600, 90);
        this.B3Id = this.AddSubPart( tsubpart1, 600, 90, 150, 35, 1);
        @this = this.detailnr;
        if (@this > -1)
        {
          let mut tsubpart2: SubPartClass =  new OfficerPartClass(-1, this.game, @this);
          this.Text2Id = this.AddSubPart( tsubpart2, 750, 0, 300, 200, 0);
        }
      }
      else
      {
        let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Swap/Appoint", 150, "You have to select an officer in the officerpool first",  this.OwnBitmap, 600, 50, true);
        this.b2bid = this.AddSubPart( tsubpart3, 600, 50, 150, 35, 0);
        let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Remove >", 150, "You have to select an officer in the officerpool first",  this.OwnBitmap, 600, 90, true);
        this.b3bid = this.AddSubPart( tsubpart4, 600, 90, 150, 35, 1);
      }
      if (this.game.EditObj.OrderUnit > -1)
      {
        if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical > -1)
        {
          @this = Math.Abs(Math.Min(0, this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical].PP));
          if (this.game.Data.RegimeObj[this.game.Data.Turn].ResPts >= @this)
          {
            let mut tsubpart: SubPartClass =  new TextButtonPartClass("< Relieve (" + @this.ToString() + "pp)", 150, "Click to relieve the leader in the unit from command without replacing him",  this.OwnBitmap, 600, 130);
            this.B4Id = this.AddSubPart( tsubpart, 600, 130, 150, 35, 1);
          }
          else
          {
            let mut tsubpart: SubPartClass =  new TextButtonPartClass("< Relieve", 150, "You dont have the PP neccessary to relieve this commander",  this.OwnBitmap, 600, 130, true);
            this.b4bid = this.AddSubPart( tsubpart, 600, 130, 150, 35, 1);
          }
        }
        else
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("< Relieve", 150, "Selected HQ has no leader so you cannot use relieve order",  this.OwnBitmap, 600, 130, true);
          this.b4bid = this.AddSubPart( tsubpart, 600, 130, 150, 35, 1);
        }
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
    }

    pub fn PopUpRefresh()
    {
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
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
            if (num1 == this.Text1Id || num1 == this.Text2Id)
            {
              let mut num2: i32 =  x - this.SubPartX[index1];
              let mut num3: i32 =  y - this.SubPartY[index1];
              if (num2 > 50 & num3 > 107 & num2 < 350 & num3 < 177)
              {
                this.game.EditObj.PopupValue = 4;
                index2: i32;
                index3: i32;
                this.game.EditObj.HandCard = this.game.Data.HistoricalUnitObj[index2].HandCard[index3];
                this.game.EditObj.TempHisUnit = this.SubPartList[index1].GetSelect();
                windowReturnClass.AddCommand(5, 10);
                this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num2 > 45 & num3 > 0 & num2 < 131 & num3 < 86)
              {
                this.game.EditObj.PopupValue = 4;
                this.game.EditObj.TempHisUnit = this.SubPartList[index1].GetSelect();
                windowReturnClass.AddCommand(5, 10);
                this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else
            {
              if (num1 == this.B1Id)
              {
                this.game.ProcessingObj.RecruitOfficer(this.game.Data.Turn, true);
                this.detailnr = this.game.Data.HistoricalUnitCounter;
                this.RemoveSubPart(this.OptionsListId);
                this.OptionsListId = 0;
                this.dostuff();
                windowReturnClass.AddCommand(4, 18);
                windowReturnClass.AddCommand(4, 66);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B2Id)
              {
                this.game.ProcessingObj.SwapOfficer(this.game.Data.Turn, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical, this.detailnr, this.game.EditObj.OrderUnit);
                this.detailnr = this.game.Data.HistoricalUnitCounter;
                this.RemoveSubPart(this.OptionsListId);
                this.OptionsListId = 0;
                this.detailnr = -1;
                this.dostuff();
                windowReturnClass.AddCommand(4, 18);
                windowReturnClass.AddCommand(4, 66);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B3Id)
              {
                if (Interaction.MsgBox( "Are you sure?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                {
                  let mut num4: i32 =   Interaction.MsgBox( "Officer is removed from the officerpool.", Title: ( "Shadow Empire : Planetary Conquest"));
                  this.game.Data.RemoveHistoricalUnit(this.detailnr);
                  this.detailnr = -1;
                  this.RemoveSubPart(this.OptionsListId);
                  this.OptionsListId = 0;
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
              }
              else
              {
                if (num1 == this.B4Id)
                {
                  this.game.ProcessingObj.SwapOfficer(this.game.Data.Turn, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical, -1, this.game.EditObj.OrderUnit);
                  this.detailnr = this.game.Data.HistoricalUnitCounter;
                  this.RemoveSubPart(this.OptionsListId);
                  this.OptionsListId = 0;
                  this.detailnr = -1;
                  this.dostuff();
                  windowReturnClass.AddCommand(4, 18);
                  windowReturnClass.AddCommand(4, 66);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.OptionsListId)
                {
                  let mut num5: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                  this.SubPartFlag[index1] = true;
                  if (num5 > -1)
                  {
                    this.detailnr = num5;
                    this.dostuff();
                  }
                  if (num5 == -2)
                  {
                    this.detailnr = -1;
                    this.dostuff();
                  }
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
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
