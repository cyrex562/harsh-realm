// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.PlayExtraWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class PlayExtraWindowClass : WindowClass
  {
     UnitHeaderId: i32;
     UnitsfId: i32;
     OfficerId: i32;
     QuickInfoId: i32;
     StatsId: i32;
     ReportId: i32;
     DisbandId: i32;
     SfButtonId: i32;
     DisbandTroopsId: i32;
     DisbandItemsId: i32;
     ItemsButtonId: i32;
     CurrentView: i32;
     NewSFId: i32;
     SOA1Id: i32;
     SOA2Id: i32;
     SOA3Id: i32;
     SOA4Id: i32;
     SOATextId: i32;
     SOB1Id: i32;
     SOB2Id: i32;
     SOB3Id: i32;
     SOB4Id: i32;
     SOBTextid: i32;
     OptionsListId: i32;
     ATListClass OptionsListObj;
     OptionsList2Id: i32;
     ListClass OptionsList2Obj;
     AttackPercentId: i32;
     DefendPercentID: i32;
     ReplacementID: i32;
     RequestSupID: i32;
     ATListClass ListObj;
     CancelButtonId: i32;
     CancelTextId: i32;
     OrderTextId: i32;
     AsId: i32;
     OrderText2Id: i32;
     OrderText3Id: i32;
     OrderText4Id: i32;
     ClearId: i32;
     ClearTextId: i32;
     OrderUpId: i32;
     OrderDownId: i32;
     OrderOkId: i32;
     OrderOkTextId: i32;
     OrderOk2Id: i32;
     DitchId: i32;
     ReserveId: i32;
     OrderOk2TextId: i32;
     ExtraId: i32;
     SliderButtonId: i32;
     AcapId: i32;
     DefId: i32;
     SupplyId: i32;
     InterceptId: i32;
     detail1: i32;
     detail2: i32;
     detail2a: i32;
     detailSkill: i32;
     detail2b: i32;
     detail1b: i32;
     SubPartClass listy;

    pub PlayExtraWindowClass( tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 =  -1, let mut sy: i32 =  -1)
      : base( tGame, tGame.ScreenWidth, 210, 99, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      this.CurrentView = 1;
      if (this.game.EditObj.SetViewMode > 0)
      {
        this.CurrentView = this.game.EditObj.SetViewMode;
        this.game.EditObj.SetViewMode = 0;
      }
      this.detail1 = -1;
      this.detail2 = -1;
      this.detail2a = -1;
      if (this.game.SelectX > -1 & this.game.SelectY > -1 & this.game.EditObj.UnitSelected == -1 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter > -1)
        this.game.EditObj.UnitSelected = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitList[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter];
      this.dostuff();
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      if (!this.formref.RightMousePressed)
        return;
      this.game.EditObj.TipTitle = "UNIT WINDOW";
      this.game.EditObj.TipText = "You can inspect your units here.";
    }

    pub WindowDescription: String(x: i32, y: i32)
    {
      str: String = "";
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
          return this.MouseText[index];
      }
      let mut num: i32 =   Math.Round( (this.game.ScreenWidth - 1024) / 2.0);
      return str;
    }

    pub fn DoRefresh() => this.dostuff();

    pub fn dostuff_backup()
    {
      if (this.UnitHeaderId > 0)
        this.RemoveSubPart(this.UnitHeaderId);
      if (this.UnitsfId > 0)
        this.RemoveSubPart(this.UnitsfId);
      if (this.SfButtonId > 0)
        this.RemoveSubPart(this.SfButtonId);
      if (this.ItemsButtonId > 0)
        this.RemoveSubPart(this.ItemsButtonId);
      if (this.StatsId > 0)
        this.RemoveSubPart(this.StatsId);
      if (this.ReportId > 0)
        this.RemoveSubPart(this.ReportId);
      if (this.NewSFId > 0)
        this.RemoveSubPart(this.NewSFId);
      if (this.SOA1Id > 0)
        this.RemoveSubPart(this.SOA1Id);
      if (this.SOA2Id > 0)
        this.RemoveSubPart(this.SOA2Id);
      if (this.SOA3Id > 0)
        this.RemoveSubPart(this.SOA3Id);
      if (this.SOA4Id > 0)
        this.RemoveSubPart(this.SOA4Id);
      if (this.SOB1Id > 0)
        this.RemoveSubPart(this.SOB1Id);
      if (this.SOB2Id > 0)
        this.RemoveSubPart(this.SOB2Id);
      if (this.SOB3Id > 0)
        this.RemoveSubPart(this.SOB3Id);
      if (this.SOB4Id > 0)
        this.RemoveSubPart(this.SOB4Id);
      if (this.SOATextId > 0)
        this.RemoveSubPart(this.SOATextId);
      if (this.AsId > 0)
        this.RemoveSubPart(this.AsId);
      if (this.SOBTextid > 0)
        this.RemoveSubPart(this.SOBTextid);
      if (this.DisbandId > 0)
        this.RemoveSubPart(this.DisbandId);
      if (this.ReplacementID > 0)
        this.RemoveSubPart(this.ReplacementID);
      if (this.DefendPercentID > 0)
        this.RemoveSubPart(this.DefendPercentID);
      if (this.RequestSupID > 0)
        this.RemoveSubPart(this.RequestSupID);
      if (this.OfficerId > 0)
        this.RemoveSubPart(this.OfficerId);
      if (this.QuickInfoId > 0)
        this.RemoveSubPart(this.QuickInfoId);
      if (this.CancelButtonId > 0)
        this.RemoveSubPart(this.CancelButtonId);
      if (this.CancelTextId > 0)
        this.RemoveSubPart(this.CancelTextId);
      if (this.OrderTextId > 0)
        this.RemoveSubPart(this.OrderTextId);
      if (this.OrderText2Id > 0)
        this.RemoveSubPart(this.OrderText2Id);
      if (this.OrderText3Id > 0)
        this.RemoveSubPart(this.OrderText3Id);
      if (this.ClearId > 0)
        this.RemoveSubPart(this.ClearId);
      if (this.ClearTextId > 0)
        this.RemoveSubPart(this.ClearTextId);
      if (this.OrderUpId > 0)
        this.RemoveSubPart(this.OrderUpId);
      if (this.OrderDownId > 0)
        this.RemoveSubPart(this.OrderDownId);
      if (this.OrderOkId > 0)
        this.RemoveSubPart(this.OrderOkId);
      if (this.OrderOkTextId > 0)
        this.RemoveSubPart(this.OrderOkTextId);
      if (this.OrderOk2Id > 0)
        this.RemoveSubPart(this.OrderOk2Id);
      if (this.OrderOk2TextId > 0)
        this.RemoveSubPart(this.OrderOk2TextId);
      if (this.ExtraId > 0)
        this.RemoveSubPart(this.ExtraId);
      if (this.SliderButtonId > 0)
        this.RemoveSubPart(this.SliderButtonId);
      if (this.DisbandTroopsId > 0)
        this.RemoveSubPart(this.DisbandTroopsId);
      if (this.DisbandItemsId > 0)
        this.RemoveSubPart(this.DisbandItemsId);
      if (this.AcapId > 0)
        this.RemoveSubPart(this.AcapId);
      if (this.DefId > 0)
        this.RemoveSubPart(this.DefId);
      if (this.SupplyId > 0)
        this.RemoveSubPart(this.SupplyId);
      if (this.InterceptId > 0)
        this.RemoveSubPart(this.InterceptId);
      if (this.game.EditObj.UnitSelected > -1)
      {
        if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount + this.game.Data.UnitObj[this.game.EditObj.UnitSelected].PassengerCounter <= 6 && this.OptionsListId > 0)
        {
          this.RemoveSubPart(this.OptionsListId);
          this.OptionsListId = 0;
        }
      }
      else if (this.OptionsListId > 0)
      {
        this.RemoveSubPart(this.OptionsListId);
        this.OptionsListId = 0;
      }
      this.detail1b = 0;
      this.detail2b = 0;
      this.NewBackGroundAndClearAll(this.game.ScreenWidth, 200, -1);
      Graphics.FromImage((Image) this.OwnBitmap);
      let mut num: i32 =  0;
      if (this.game.EditObj.OrderType < 1)
        num = 1;
      if (this.game.EditObj.OrderType == 1)
        num = 1;
      if (this.game.EditObj.OrderType == 9 & this.game.EditObj.OrderTarget == -1)
        num = 1;
      if (this.game.EditObj.OrderType == 18 & this.game.EditObj.OrderTarget == -1)
        num = 1;
      if (this.game.EditObj.OrderType == 18 & this.game.EditObj.TargetX == -1)
        num = 1;
      if (this.game.EditObj.OrderType == 3)
        num = 1;
      if (this.game.EditObj.OrderType == 19)
        num = 1;
      if (this.game.EditObj.OrderType == 14)
        num = 1;
      if (this.game.EditObj.OrderType == 15)
        num = 1;
      if (this.game.EditObj.OrderType == 2)
        num = 1;
      if (this.game.EditObj.OrderType == 12)
        num = 1;
      if (this.game.EditObj.OrderType == 11)
        num = 1;
      if (this.game.EditObj.OrderType == 13)
        num = 1;
      if (this.game.EditObj.OrderType == 21)
        num = 1;
      if (this.game.EditObj.OrderType == 20)
        num = 1;
      if (this.game.EditObj.OrderType == 37)
        num = 1;
      if (this.game.EditObj.OrderType == 36)
        num = 1;
      if (this.game.EditObj.OrderType == 35)
        num = 1;
      if (this.game.EditObj.OrderType == 39)
        num = 1;
      if (this.game.EditObj.OrderType == 40)
        num = 1;
      if (this.game.EditObj.HideUnit != 0)
        return;
      num = 0;
    }

    pub fn dostuff()
    {
      if (this.UnitHeaderId > 0)
        this.RemoveSubPart(this.UnitHeaderId);
      if (this.UnitsfId > 0)
        this.RemoveSubPart(this.UnitsfId);
      if (this.SfButtonId > 0)
        this.RemoveSubPart(this.SfButtonId);
      if (this.ItemsButtonId > 0)
        this.RemoveSubPart(this.ItemsButtonId);
      if (this.StatsId > 0)
        this.RemoveSubPart(this.StatsId);
      if (this.ReportId > 0)
        this.RemoveSubPart(this.ReportId);
      if (this.NewSFId > 0)
        this.RemoveSubPart(this.NewSFId);
      if (this.SOA1Id > 0)
        this.RemoveSubPart(this.SOA1Id);
      if (this.SOA2Id > 0)
        this.RemoveSubPart(this.SOA2Id);
      if (this.SOA3Id > 0)
        this.RemoveSubPart(this.SOA3Id);
      if (this.SOA4Id > 0)
        this.RemoveSubPart(this.SOA4Id);
      if (this.SOB1Id > 0)
        this.RemoveSubPart(this.SOB1Id);
      if (this.SOB2Id > 0)
        this.RemoveSubPart(this.SOB2Id);
      if (this.SOB3Id > 0)
        this.RemoveSubPart(this.SOB3Id);
      if (this.SOB4Id > 0)
        this.RemoveSubPart(this.SOB4Id);
      if (this.SOATextId > 0)
        this.RemoveSubPart(this.SOATextId);
      if (this.AsId > 0)
        this.RemoveSubPart(this.AsId);
      if (this.SOBTextid > 0)
        this.RemoveSubPart(this.SOBTextid);
      if (this.DisbandId > 0)
        this.RemoveSubPart(this.DisbandId);
      if (this.ReplacementID > 0)
        this.RemoveSubPart(this.ReplacementID);
      if (this.DefendPercentID > 0)
        this.RemoveSubPart(this.DefendPercentID);
      if (this.RequestSupID > 0)
        this.RemoveSubPart(this.RequestSupID);
      if (this.OfficerId > 0)
        this.RemoveSubPart(this.OfficerId);
      if (this.QuickInfoId > 0)
        this.RemoveSubPart(this.QuickInfoId);
      if (this.CancelButtonId > 0)
        this.RemoveSubPart(this.CancelButtonId);
      if (this.CancelTextId > 0)
        this.RemoveSubPart(this.CancelTextId);
      if (this.OrderTextId > 0)
        this.RemoveSubPart(this.OrderTextId);
      if (this.OrderText2Id > 0)
        this.RemoveSubPart(this.OrderText2Id);
      if (this.OrderText3Id > 0)
        this.RemoveSubPart(this.OrderText3Id);
      if (this.ClearId > 0)
        this.RemoveSubPart(this.ClearId);
      if (this.ClearTextId > 0)
        this.RemoveSubPart(this.ClearTextId);
      if (this.OrderUpId > 0)
        this.RemoveSubPart(this.OrderUpId);
      if (this.OrderDownId > 0)
        this.RemoveSubPart(this.OrderDownId);
      if (this.OrderOkId > 0)
        this.RemoveSubPart(this.OrderOkId);
      if (this.OrderOkTextId > 0)
        this.RemoveSubPart(this.OrderOkTextId);
      if (this.OrderOk2Id > 0)
        this.RemoveSubPart(this.OrderOk2Id);
      if (this.OrderOk2TextId > 0)
        this.RemoveSubPart(this.OrderOk2TextId);
      if (this.ExtraId > 0)
        this.RemoveSubPart(this.ExtraId);
      if (this.SliderButtonId > 0)
        this.RemoveSubPart(this.SliderButtonId);
      if (this.DitchId > 0)
        this.RemoveSubPart(this.DitchId);
      if (this.ReserveId > 0)
        this.RemoveSubPart(this.ReserveId);
      if (this.DisbandTroopsId > 0)
        this.RemoveSubPart(this.DisbandTroopsId);
      if (this.DisbandItemsId > 0)
        this.RemoveSubPart(this.DisbandItemsId);
      if (this.AcapId > 0)
        this.RemoveSubPart(this.AcapId);
      if (this.DefId > 0)
        this.RemoveSubPart(this.DefId);
      if (this.SupplyId > 0)
        this.RemoveSubPart(this.SupplyId);
      if (this.InterceptId > 0)
        this.RemoveSubPart(this.InterceptId);
      if (this.game.EditObj.UnitSelected > -1)
      {
        if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount + this.game.Data.UnitObj[this.game.EditObj.UnitSelected].PassengerCounter <= 6 && this.OptionsListId > 0)
        {
          this.RemoveSubPart(this.OptionsListId);
          this.OptionsListId = 0;
        }
      }
      else if (this.OptionsListId > 0)
      {
        this.RemoveSubPart(this.OptionsListId);
        this.OptionsListId = 0;
      }
      this.detail1b = 0;
      this.detail2b = 0;
      this.NewBackGroundAndClearAll(this.game.ScreenWidth, 210, -1);
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      let mut num1: i32 =  0;
      if (this.game.EditObj.OrderType < 1)
        num1 = 1;
      if (this.game.EditObj.OrderType == 1)
        num1 = 1;
      if (this.game.EditObj.OrderType == 9 & this.game.EditObj.OrderTarget == -1)
        num1 = 1;
      if (this.game.EditObj.OrderType == 18 & this.game.EditObj.OrderTarget == -1)
        num1 = 1;
      if (this.game.EditObj.OrderType == 18 & this.game.EditObj.TargetX == -1)
        num1 = 1;
      if (this.game.EditObj.OrderType == 3)
        num1 = 1;
      if (this.game.EditObj.OrderType == 19)
        num1 = 1;
      if (this.game.EditObj.OrderType == 14)
        num1 = 1;
      if (this.game.EditObj.OrderType == 15)
        num1 = 1;
      if (this.game.EditObj.OrderType == 2)
        num1 = 1;
      if (this.game.EditObj.OrderType == 12)
        num1 = 1;
      if (this.game.EditObj.OrderType == 11)
        num1 = 1;
      if (this.game.EditObj.OrderType == 13)
        num1 = 1;
      if (this.game.EditObj.OrderType == 21)
        num1 = 1;
      if (this.game.EditObj.OrderType == 20)
        num1 = 1;
      if (this.game.EditObj.OrderType == 37)
        num1 = 1;
      if (this.game.EditObj.OrderType == 36)
        num1 = 1;
      if (this.game.EditObj.OrderType == 35)
        num1 = 1;
      if (this.game.EditObj.OrderType == 39)
        num1 = 1;
      if (this.game.EditObj.OrderType == 40)
        num1 = 1;
      if (this.game.EditObj.HideUnit == 0)
        num1 = 0;
      this.ClearMouse();
      let mut h: i32 =  200;
      let mut x1: i32 =   Math.Round( (this.game.ScreenWidth - 1024) / 2.0);
      let mut num2: i32 =  10;
      DrawMod.DrawBlock( Expression, 0, 0, this.game.ScreenWidth, num2,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  this.game.VicColor4.A);
      let mut num3: i32 =  0;
      Rectangle trect1;
      Rectangle trect2;
      if (num1 == 1 && this.game.EditObj.UnitSelected > -1 & this.game.EditObj.UnitSelected <= this.game.Data.UnitCounter && this.game.HandyFunctionsObj.CanWeSeeUnit(this.game.EditObj.UnitSelected, this.game.Data.Turn) > 0)
      {
        let mut num4: i32 =  x1 + 50;
        DrawMod.DrawBlock( Expression, num4 + 295, num2, 48, h,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  this.game.VicColor4.A);
        DrawMod.drawLine( Expression, 0, num2, num4 + 295, num2,  this.game.VicColor6.R,  this.game.VicColor6.G,  this.game.VicColor6.B,  this.game.VicColor6.A);
        DrawMod.drawLine( Expression, num4 + 295 + 48, num2, this.game.ScreenWidth, num2,  this.game.VicColor6.R,  this.game.VicColor6.G,  this.game.VicColor6.B,  this.game.VicColor6.A);
        DrawMod.drawLine( Expression, num4 + 295, num2, num4 + 295, num2 + h,  this.game.VicColor6.R,  this.game.VicColor6.G,  this.game.VicColor6.B,  this.game.VicColor6.A);
        DrawMod.drawLine( Expression, num4 + 343, num2, num4 + 343, num2 + h,  this.game.VicColor6.R,  this.game.VicColor6.G,  this.game.VicColor6.B,  this.game.VicColor6.A);
        c2_1: Color = Color.FromArgb(0, 100, 100, 100);
        c1_1: Color = Color.FromArgb(100, 100, 100, 100);
        if (this.game.EditObj.SetViewMode2 == 0)
        {
          c2_1 = Color.FromArgb(60,  this.game.viccolor7.R,  this.game.viccolor7.G,  this.game.viccolor7.B);
          c1_1 = Color.FromArgb(150,  this.game.viccolor7.R,  this.game.viccolor7.G,  this.game.viccolor7.B);
        }
        DrawMod.DrawBlockGradient2( Expression, num4 + 294 + 8, 42, 35, 40, c1_1, c2_1);
        DrawMod.DrawTextVic( Expression, "SFs", this.game.VicFont5, num4 + 294 + 5 + 10, 56, this.game.VicColor1, this.game.VicColor1Shade);
        trect1 = Rectangle::new(num4 + 294 + 8, 42, 35, 40);
        this.AddMouse( trect1, "", "View subformations", 1);
        c2_2: Color = Color.FromArgb(0, 100, 100, 100);
        c1_2: Color = Color.FromArgb(100, 100, 100, 100);
        if (this.game.EditObj.SetViewMode2 == 1)
        {
          c2_2 = Color.FromArgb(60,  this.game.viccolor7.R,  this.game.viccolor7.G,  this.game.viccolor7.B);
          c1_2 = Color.FromArgb(150,  this.game.viccolor7.R,  this.game.viccolor7.G,  this.game.viccolor7.B);
        }
        DrawMod.DrawBlockGradient2( Expression, num4 + 294 + 8, 89, 35, 40, c1_2, c2_2);
        DrawMod.DrawTextVic( Expression, "LOG", this.game.VicFont5, num4 + 294 + 3 + 10, 105, this.game.VicColor1, this.game.VicColor1Shade);
        trect1 = Rectangle::new(num4 + 294 + 8, 89, 35, 40);
        let mut trect3: &Rectangle = &trect1
        this.AddMouse( trect3, "", "View logistical statistics", 2);
        c2_3: Color = Color.FromArgb(0, 100, 100, 100);
        c1_3: Color = Color.FromArgb(100, 100, 100, 100);
        if (this.game.EditObj.SetViewMode2 == 2)
        {
          c2_3 = Color.FromArgb(60,  this.game.viccolor7.R,  this.game.viccolor7.G,  this.game.viccolor7.B);
          c1_3 = Color.FromArgb(150,  this.game.viccolor7.R,  this.game.viccolor7.G,  this.game.viccolor7.B);
        }
        DrawMod.DrawBlockGradient2( Expression, num4 + 294 + 8, 138, 35, 40, c1_3, c2_3);
        DrawMod.DrawTextVic( Expression, "ANL", this.game.VicFont5, num4 + 294 + 3 + 10, 154, this.game.VicColor1, this.game.VicColor1Shade);
        trect1 = Rectangle::new(num4 + 294 + 8, 138, 35, 40);
        trect2 = trect1;
        this.AddMouse( trect2, "", "View unit analysis statistics", 3);
        x1 = num4 - 50;
        let mut tsubpart: SubPartClass =  new UnitHeaderPartClass(this.game.EditObj.UnitSelected, this.game);
        this.UnitHeaderId = this.AddSubPart( tsubpart, x1, num2 + 0, 280, 200, 0);
        num3 = 1;
      }
      if (num3 == 0)
        DrawMod.drawLine( Expression, 0, num2, this.game.ScreenWidth, num2,  this.game.VicColor6.R,  this.game.VicColor6.G,  this.game.VicColor6.B,  this.game.VicColor6.A);
      Coordinate coordinate = Coordinate::new();
      if (this.game.EditObj.UnitSelected > -1)
      {
        if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn | this.game.Data.Round == 0)
          coordinate.x = 3;
        else
          coordinate = this.game.HandyFunctionsObj.GetReconMinusHide(this.game.EditObj.UnitSelected, this.game.Data.Turn);
      }
      else
        coordinate.x = 3;
      tname: String;
      bitmap: Bitmap;
      if (num1 == 1 && this.game.EditObj.UnitSelected > -1 & this.game.EditObj.UnitSelected <= this.game.Data.UnitCounter && this.game.HandyFunctionsObj.CanWeSeeUnit(this.game.EditObj.UnitSelected, this.game.Data.Turn) > 0 & coordinate.x >= 2)
      {
        let mut num5: i32 =  x1 - 691;
        let mut num6: i32 =  num2 + 35;
        if (this.game.Data.Turn == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime | this.game.Data.Round == 0 | !this.game.Data.FOWOn)
        {
          Conversions.ToInteger(this.game.HandyFunctionsObj.GetUnitPeople(this.game.EditObj.UnitSelected));
          let mut regime: i32 =  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime;
          tname = Strings.Trim(Conversion.Str( (100 - this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SODefendPercent))) + "%";
          let mut tsubpart1: SubPartClass =  TextPartClass::new("", this.game.GameFont2, 49, 23, true, 0, "Retreat Percentage. Unit retreats if this % of losses has been suffered. 25%=retreat quickly, 100%=to death");
          this.DefId = this.AddSubPart( tsubpart1, num5 + 975, num6 + 91, 49, 23, 0);
           let mut local1: &Graphics = &Expression;
          trect1 = Rectangle::new(num5 + 975, num6 + 77, 49, 14);
          let mut rect1_1: &Rectangle = &trect1
          trect2 = Rectangle::new(num5 + 975, num6 + 77 + 14, 49, 23);
          let mut rect2_1: &Rectangle = &trect2
          txt2_1: String = tname;
          DrawMod.MakeFullBoxVic2( local1, rect1_1, "LOSS", rect2_1, txt2_1);
          if (this.game.HandyFunctionsObj.HasUnitAirSF(this.game.EditObj.UnitSelected))
          {
            tname = this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOInterceptRdnStop != 100 ? Strings.Trim(Conversion.Str( this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOInterceptRdnStop)) : "Don't";
            let mut tsubpart2: SubPartClass =  TextPartClass::new("", this.game.GameFont2, 49, 23, true, 0, "Intercepts only if readiness is greater than specified. You can put on don't, which forbids intercept.");
            this.InterceptId = this.AddSubPart( tsubpart2, num5 + 975, num6 + 3 + 28 + 23, 49, 23, 0);
             let mut local2: &Graphics = &Expression;
            trect1 = Rectangle::new(num5 + 975, num6 + 3 + 14 + 23, 49, 14);
            let mut rect1_2: &Rectangle = &trect1
            trect2 = Rectangle::new(num5 + 975, num6 + 3 + 28 + 23, 49, 23);
            let mut rect2_2: &Rectangle = &trect2
            txt2_2: String = tname;
            DrawMod.MakeFullBoxVic2( local2, rect1_2, "INTC", rect2_2, txt2_2);
          }
          if (this.game.Data.ASOn & this.game.HandyFunctionsObj.GetUnitAS(this.game.EditObj.UnitSelected) > 0)
          {
            tname = !this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SODoAS ? "No" : "Yes".to_owned();
            let mut tsubpart3: SubPartClass =  TextPartClass::new("", this.game.GameFont2, 49, 23, true, 0, "Wheter or not the unit will do Anti Supply damage on enemy hexes.");
            this.AsId = this.AddSubPart( tsubpart3, num5 + 981, num6 + 16, 49, 23, 0);
             let mut local3: &Graphics = &Expression;
            trect1 = Rectangle::new(num5 + 975, num6 + 2, 49, 14);
            let mut rect1_3: &Rectangle = &trect1
            trect2 = Rectangle::new(num5 + 975, num6 + 2 + 14, 49, 23);
            let mut rect2_3: &Rectangle = &trect2
            txt2_3: String = tname;
            DrawMod.MakeFullBoxVic2( local3, rect1_3, "DOAS", rect2_3, txt2_3);
          }
        }
        if (this.game.Data.Turn == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime | this.game.Data.Round == 0 | !this.game.Data.FOWOn | this.game.Data.Turn == this.game.Data.RegimeObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime].UberRegime && this.game.Data.Turn > -1)
        {
          tname = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOSupReqPercent)) + "%";
          let mut tsubpart: SubPartClass =  TextPartClass::new("", this.game.GameFont2, 49, 23, true, 0, "Supply Rationing. The % of supply requested that will be alloted by superior hq.");
          this.SupplyId = this.AddSubPart( tsubpart, num5 + 975, num6 + 4 + 56 + 69, 49, 23, 0);
           let mut local: &Graphics = &Expression;
          trect1 = Rectangle::new(num5 + 975, num6 + 4 + 42 + 69, 49, 14);
          let mut rect1: &Rectangle = &trect1
          trect2 = Rectangle::new(num5 + 975, num6 + 4 + 56 + 69, 49, 23);
          let mut rect2: &Rectangle = &trect2
          txt2: String = tname;
          DrawMod.MakeFullBoxVic2( local, rect1, "SUPP", rect2, txt2);
        }
        let mut num7: i32 =  num5 + 691;
        let mut num8: i32 =  num6 - 35;
        tlistselect: i32;
        SubPartClass tsubpart4;
        if (this.game.EditObj.SetViewMode2 == 0)
        {
          num7 += 50;
          let mut num9: i32 =  0;
          if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount + this.game.Data.UnitObj[this.game.EditObj.UnitSelected].PassengerCounter > 6)
            num9 = 1;
          let mut index: i32 =  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical;
          if (index > -1)
          {
            if (this.game.Data.HistoricalUnitObj[index].CommanderSpriteID > -1)
              num9 = 1;
            else
              index = -1;
          }
          if (num9 == 1)
          {
            this.OptionsListObj = ATListClass::new();
            tlistselect = -1;
            let mut num10: i32 =  -1;
            let mut sfCount: i32 =  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount;
            for (let mut tdata: i32 =  0; tdata <= sfCount; tdata += 1)
            {
              let mut sf: i32 =  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFList[tdata];
              let mut type: i32 =  this.game.Data.SFObj[sf].Type;
              let mut qty: i32 =  this.game.Data.SFObj[sf].Qty;
              if (this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].Ratio > 0)
                qty *= this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].Ratio;
              num10 += 1;
              if (this.detail1 == sf)
                tlistselect = num10;
              tname = Strings.Trim(Conversion.Str( qty)) + "x " + this.game.Data.SFTypeObj[type].Name;
              if (this.game.Data.SFObj[sf].MoveType > -1)
                tname = tname + " (" + this.game.Data.TempString[this.game.Data.SFObj[sf].MoveType] + ")";
              tvalue: String = Strings.Trim(Conversion.Str( this.game.Data.SFObj[sf].Ap));
              tvalue2: String = Strings.Trim(Conversion.Str( this.game.Data.SFObj[sf].Rdn));
              tvalue3: String = Strings.Trim(Conversion.Str( this.game.Data.SFObj[sf].Xp));
              tvalue4: String = Strings.Left(this.game.Data.PeopleObj[this.game.Data.SFObj[sf].People].Name, 3);
              this.OptionsListObj.add(tname, tdata, tvalue, tvalue2, tvalue3, tvalue4);
            }
            if (this.OptionsListId > 0)
            {
              this.SubPartList[this.SubpartNr(this.OptionsListId)].Refresh(this.OptionsListObj, tlistselect);
              this.SubPartFlag[this.SubpartNr(this.OptionsListId)] = true;
            }
            else if (index > -1)
            {
              tsubpart4 =  new ATListSubPartClass(this.OptionsListObj, 9, 310, tlistselect, this.game, tHeader: "Subformations                     AP    RDN   XP    PPL", tHeaderCenter: false, tShowPair: true, tValueWidth: 130, tbackbitmap: ( this.OwnBitmap), bbx: (num7 + 665), bby: (num8 + 5));
              this.OptionsListId = this.AddSubPart( tsubpart4, num7 + 665, num8 + 5, 310, 192, 0);
            }
            else
            {
              tsubpart4 =  new ATListSubPartClass(this.OptionsListObj, 9, 590, tlistselect, this.game, tHeader: "Subformations", tShowPair: true, tValueWidth: 300, tbackbitmap: ( this.OwnBitmap), bbx: (num7 + 355), bby: (num8 + 5));
              this.OptionsListId = this.AddSubPart( tsubpart4, num7 + 355, num8 + 5, 590, 192, 0);
            }
            if (index > -1)
            {
              tsubpart4 =  new OfficerPartClass(this.game.EditObj.UnitSelected, this.game);
              this.OfficerId = this.AddSubPart( tsubpart4, num7 + 355, num8 + 5, 300, 200, 0);
            }
          }
          else
          {
            let mut tsubpart5: SubPartClass =  new UnitSFPartClass(this.game.EditObj.UnitSelected, this.game);
            this.UnitsfId = this.AddSubPart( tsubpart5, num7 + 350, num8 + 0, 620, 200, 0);
          }
        }
        let mut unitSelected: i32 =  this.game.EditObj.UnitSelected;
        if (this.game.EditObj.SetViewMode2 == 1)
        {
          if (this.OptionsListId > 0)
          {
            this.RemoveSubPart(this.OptionsListId);
            this.OptionsListId = 0;
          }
          if (!this.game.Data.FOWOn | this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.Turn, this.game.Data.UnitObj[unitSelected].Regime))
          {
            let mut x2: i32 =  num7 + 360 + 50;
            let mut width: i32 =  80;
            if (this.game.Data.UnitObj[unitSelected].IsHQ)
            {
              str1: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[unitSelected].LandCap));
               let mut local4: &Graphics = &Expression;
              trect1 = Rectangle::new(x2, 25, width, 14);
              let mut rect1_4: &Rectangle = &trect1
              trect2 = Rectangle::new(x2, 39, width, 23);
              let mut rect2_4: &Rectangle = &trect2
              txt2_4: String = str1;
              DrawMod.MakeFullBoxVic2( local4, rect1_4, "LAND CAP", rect2_4, txt2_4);
              str2: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[unitSelected].NavyCap));
               let mut local5: &Graphics = &Expression;
              trect1 = Rectangle::new(x2, 70, width, 14);
              let mut rect1_5: &Rectangle = &trect1
              trect2 = Rectangle::new(x2, 84, width, 23);
              let mut rect2_5: &Rectangle = &trect2
              txt2_5: String = str2;
              DrawMod.MakeFullBoxVic2( local5, rect1_5, "NAVY CAP", rect2_5, txt2_5);
              str3: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[unitSelected].AirCap));
               let mut local6: &Graphics = &Expression;
              trect1 = Rectangle::new(x2, 115, width, 14);
              let mut rect1_6: &Rectangle = &trect1
              trect2 = Rectangle::new(x2, 129, width, 23);
              let mut rect2_6: &Rectangle = &trect2
              txt2_6: String = str3;
              DrawMod.MakeFullBoxVic2( local6, rect1_6, "RAIL CAP", rect2_6, txt2_6);
              x2 += 100;
            }
            str4: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[unitSelected].Supply));
            str5: String;
            if (this.game.HandyFunctionsObj.UnitSupplyUse(unitSelected) > 0)
            {
              float Number =  Math.Round( ( this.game.Data.UnitObj[unitSelected].Supply /  this.game.HandyFunctionsObj.UnitSupplyUse(unitSelected)), 1);
              if ( Number > 99.0)
                Number = 99f;
              str5 = Strings.Trim(Conversion.Str( Number));
            }
            else
              str5 = ">99";
             let mut local7: &Graphics = &Expression;
            trect1 = Rectangle::new(x2, 25, width, 14);
            let mut rect1_7: &Rectangle = &trect1
            trect2 = Rectangle::new(x2, 39, width, 23);
            let mut rect2_7: &Rectangle = &trect2
            txt2_7: String = str4;
            DrawMod.MakeFullBoxVic2( local7, rect1_7, "SUPPLY STOCK", rect2_7, txt2_7);
            str6: String = str5;
             let mut local8: &Graphics = &Expression;
            trect1 = Rectangle::new(x2, 70, width, 14);
            let mut rect1_8: &Rectangle = &trect1
            trect2 = Rectangle::new(x2, 84, width, 23);
            let mut rect2_8: &Rectangle = &trect2
            txt2_8: String = str6;
            DrawMod.MakeFullBoxVic2( local8, rect1_8, "ROUNDS LEFT", rect2_8, txt2_8);
            let mut x3: i32 =  x2 + 100;
            str7: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[unitSelected].SupplyInReq));
             let mut local9: &Graphics = &Expression;
            trect1 = Rectangle::new(x3, 25, width, 14);
            let mut rect1_9: &Rectangle = &trect1
            trect2 = Rectangle::new(x3, 39, width, 23);
            let mut rect2_9: &Rectangle = &trect2
            txt2_9: String = str7;
            DrawMod.MakeFullBoxVic2( local9, rect1_9, "REQUEST IN", rect2_9, txt2_9);
            str8: String = Strings.Trim(Conversion.Str( (this.game.Data.UnitObj[unitSelected].SupplyIn + this.game.Data.UnitObj[unitSelected].SupplyLost)));
             let mut local10: &Graphics = &Expression;
            trect1 = Rectangle::new(x3, 70, width, 14);
            let mut rect1_10: &Rectangle = &trect1
            trect2 = Rectangle::new(x3, 84, width, 23);
            let mut rect2_10: &Rectangle = &trect2
            txt2_10: String = str8;
            DrawMod.MakeFullBoxVic2( local10, rect1_10, "SUPPLY SEND", rect2_10, txt2_10);
            str9: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[unitSelected].SupplyLost));
             let mut local11: &Graphics = &Expression;
            trect1 = Rectangle::new(x3, 115, width, 14);
            let mut rect1_11: &Rectangle = &trect1
            trect2 = Rectangle::new(x3, 129, width, 23);
            let mut rect2_11: &Rectangle = &trect2
            txt2_11: String = str9;
            DrawMod.MakeFullBoxVic2( local11, rect1_11, "DESTROYED", rect2_11, txt2_11);
            tname = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[unitSelected].SupplyIn));
             let mut local12: &Graphics = &Expression;
            trect1 = Rectangle::new(x3, 160, width, 14);
            let mut rect1_12: &Rectangle = &trect1
            trect2 = Rectangle::new(x3, 174, width, 23);
            let mut rect2_12: &Rectangle = &trect2
            txt2_12: String = tname;
            DrawMod.MakeFullBoxVic2( local12, rect1_12, "SUPPLY IN", rect2_12, txt2_12);
            if (this.game.Data.UnitObj[unitSelected].IsHQ)
            {
              x3 += 100;
              str10: String = Strings.Trim(Conversion.Str( this.game.HandyFunctionsObj.GetRealHQSupplyPts(unitSelected)));
               let mut local13: &Graphics = &Expression;
              trect1 = Rectangle::new(x3, 25, width, 14);
              let mut rect1_13: &Rectangle = &trect1
              trect2 = Rectangle::new(x3, 39, width, 23);
              let mut rect2_13: &Rectangle = &trect2
              txt2_13: String = str10;
              DrawMod.MakeFullBoxVic2( local13, rect1_13, "EXCESS SUP", rect2_13, txt2_13);
              str11: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[unitSelected].SupplyReq));
               let mut local14: &Graphics = &Expression;
              trect1 = Rectangle::new(x3, 70, width, 14);
              let mut rect1_14: &Rectangle = &trect1
              trect2 = Rectangle::new(x3, 84, width, 23);
              let mut rect2_14: &Rectangle = &trect2
              txt2_14: String = str11;
              DrawMod.MakeFullBoxVic2( local14, rect1_14, "REQUEST OUT", rect2_14, txt2_14);
              str12: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[unitSelected].SupplyOut));
               let mut local15: &Graphics = &Expression;
              trect1 = Rectangle::new(x3, 115, width, 14);
              let mut rect1_15: &Rectangle = &trect1
              trect2 = Rectangle::new(x3, 129, width, 23);
              let mut rect2_15: &Rectangle = &trect2
              txt2_15: String = str12;
              DrawMod.MakeFullBoxVic2( local15, rect1_15, "SUPPLY OUT", rect2_15, txt2_15);
              tname = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[unitSelected].Reserve));
               let mut local16: &Graphics = &Expression;
              trect1 = Rectangle::new(x3, 160, width, 14);
              let mut rect1_16: &Rectangle = &trect1
              trect2 = Rectangle::new(x3, 174, width, 23);
              let mut rect2_16: &Rectangle = &trect2
              txt2_16: String = tname;
              DrawMod.MakeFullBoxVic2( local16, rect1_16, "RESERVE", rect2_16, txt2_16);
            }
            if (this.game.HandyFunctionsObj.DoesUnitUseFuel(unitSelected))
            {
              x3 += 100;
              str13: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[unitSelected].FuelUsedMove));
               let mut local17: &Graphics = &Expression;
              trect1 = Rectangle::new(x3, 25, width, 14);
              let mut rect1_17: &Rectangle = &trect1
              trect2 = Rectangle::new(x3, 39, width, 23);
              let mut rect2_17: &Rectangle = &trect2
              txt2_17: String = str13;
              DrawMod.MakeFullBoxVic2( local17, rect1_17, "FUEL MOVING", rect2_17, txt2_17);
              str14: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[unitSelected].FuelUsedAtt));
               let mut local18: &Graphics = &Expression;
              trect1 = Rectangle::new(x3, 70, width, 14);
              let mut rect1_18: &Rectangle = &trect1
              trect2 = Rectangle::new(x3, 84, width, 23);
              let mut rect2_18: &Rectangle = &trect2
              txt2_18: String = str14;
              DrawMod.MakeFullBoxVic2( local18, rect1_18, "FUEL ATTACK", rect2_18, txt2_18);
              tname = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[unitSelected].FuelUsedDef));
               let mut local19: &Graphics = &Expression;
              trect1 = Rectangle::new(x3, 115, width, 14);
              let mut rect1_19: &Rectangle = &trect1
              trect2 = Rectangle::new(x3, 129, width, 23);
              let mut rect2_19: &Rectangle = &trect2
              txt2_19: String = tname;
              DrawMod.MakeFullBoxVic2( local19, rect1_19, "FUEL DEFENSE", rect2_19, txt2_19);
            }
            if (this.game.Data.UnitObj[unitSelected].IsHQ & this.game.Data.UnitObj[unitSelected].Regime == this.game.Data.Turn)
            {
              let mut num11: i32 =  x3 + 100;
              tsubpart4 =  new TextButtonPartClass("DITCH", 100, "Allows you to ditch a specific ammount of supply points",  this.OwnBitmap, num11, 25);
              this.DitchId = this.AddSubPart( tsubpart4, num11, 25, 100, 32, 1);
              tsubpart4 =  new TextButtonPartClass("RESERVE", 100, "Set the reserve supply poammount: i32 the HQ should try to build up",  this.OwnBitmap, num11, 70);
              this.ReserveId = this.AddSubPart( tsubpart4, num11, 70, 100, 32, 1);
            }
          }
        }
        if (this.game.EditObj.SetViewMode2 == 2)
        {
          if (this.OptionsListId > 0)
          {
            this.RemoveSubPart(this.OptionsListId);
            this.OptionsListId = 0;
          }
          if (!this.game.Data.FOWOn | this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.Turn, this.game.Data.UnitObj[unitSelected].Regime))
          {
            let mut bbx: i32 =  num7 + 360 + 50;
            this.ListObj = ATListClass::new();
            int[] numArray1 = new int[100];
            int[] numArray2 = new int[100];
            this.ListObj.add("MOVETYPE", -1, "WGT", "CARRY");
            let mut sfCount: i32 =  this.game.Data.UnitObj[unitSelected].SFCount;
            for (let mut index1: i32 =  0; index1 <= sfCount; index1 += 1)
            {
              let mut sf: i32 =  this.game.Data.UnitObj[unitSelected].SFList[index1];
              let mut type: i32 =  this.game.Data.SFObj[sf].Type;
              int[] numArray3 = numArray1;
              int[] numArray4 = numArray3;
              let mut moveType1: i32 =  this.game.Data.SFTypeObj[type].MoveType;
              let mut index2: i32 =  moveType1;
              let mut num12: i32 =  numArray3[moveType1] + this.game.Data.SFObj[sf].Qty * this.game.Data.SFTypeObj[type].Weight;
              numArray4[index2] = num12;
              int[] numArray5 = numArray2;
              int[] numArray6 = numArray5;
              let mut moveType2: i32 =  this.game.Data.SFTypeObj[type].MoveType;
              let mut index3: i32 =  moveType2;
              let mut num13: i32 =  numArray5[moveType2] + this.game.Data.SFObj[sf].Qty * this.game.Data.SFTypeObj[type].CarryCap;
              numArray6[index3] = num13;
            }
            let mut index: i32 =  0;
            num14: i32;
            do
            {
              if (numArray1[index] > 0)
              {
                num14 += 1;
                this.ListObj.add(Strings.Left(this.game.Data.TempString[index], 9), -1, Strings.Trim(Conversion.Str( numArray1[index])), Strings.Trim(Conversion.Str( numArray2[index])));
              }
              index += 1;
            }
            while (index <= 99);
            if (this.game.Data.UnitObj[unitSelected].IsHQ)
            {
              let mut integer: i32 =  Conversions.ToInteger(this.game.HandyFunctionsObj.GetUnitExcessWeight(unitSelected));
              if (integer > 0)
              {
                num14 += 1;
                this.ListObj.add("Excess Sup", -1, Strings.Trim(Strings.Trim(Conversion.Str( integer))), "0");
              }
            }
            if (num14 > 0)
            {
              if (num14 > 4)
                ;
              this.listy =  new ATListSubPartClass(this.ListObj, 6, 200, tlistselect, this.game, tHeader: "Move Types", tHighlight: false, tShowPair: true, tValueWidth: 100, tbackbitmap: ( this.OwnBitmap), bbx: bbx, bby: 25);
               let mut local20: &Graphics = &Expression;
              bitmap = this.listy.Paint();
               let mut local21: &Bitmap = &bitmap;
              let mut x4: i32 =  bbx;
              DrawMod.DrawSimple( local20,  local21, x4, 25);
            }
            let mut x5: i32 =  bbx + 215;
            let mut width1: i32 =  80;
            let mut unitCarryCap1: i32 =  this.game.HandyFunctionsObj.GetUnitCarryCap(unitSelected, 0);
            str15: String = Strings.Trim(Conversion.Str( (this.game.HandyFunctionsObj.GetUnitCarryCap(unitSelected, 0) - this.game.HandyFunctionsObj.GetUnitCarryCap(unitSelected, 0, true)))) + " / " + Strings.Trim(Conversion.Str( unitCarryCap1));
             let mut local22: &Graphics = &Expression;
            trect1 = Rectangle::new(x5, 25, width1, 14);
            let mut rect1_20: &Rectangle = &trect1
            trect2 = Rectangle::new(x5, 39, width1, 23);
            let mut rect2_20: &Rectangle = &trect2
            txt2_20: String = str15;
            DrawMod.MakeFullBoxVic2( local22, rect1_20, "LAND CARRIED", rect2_20, txt2_20);
            let mut unitCarryCap2: i32 =  this.game.HandyFunctionsObj.GetUnitCarryCap(unitSelected, 1);
            str16: String = Strings.Trim(Conversion.Str( (this.game.HandyFunctionsObj.GetUnitCarryCap(unitSelected, 1) - this.game.HandyFunctionsObj.GetUnitCarryCap(unitSelected, 1, true)))) + " / " + Strings.Trim(Conversion.Str( unitCarryCap2));
             let mut local23: &Graphics = &Expression;
            trect1 = Rectangle::new(x5, 70, width1, 14);
            let mut rect1_21: &Rectangle = &trect1
            trect2 = Rectangle::new(x5, 84, width1, 23);
            let mut rect2_21: &Rectangle = &trect2
            txt2_21: String = str16;
            DrawMod.MakeFullBoxVic2( local23, rect1_21, "NAVY CARRIED", rect2_21, txt2_21);
            let mut airCarryCapPts: i32 =  this.game.HandyFunctionsObj.GetAirCarryCapPts(unitSelected);
            str17: String = Strings.Trim(Conversion.Str( Conversions.ToInteger(Operators.SubtractObject(this.game.HandyFunctionsObj.GetUnitNonSeaWeight(unitSelected, true), this.game.HandyFunctionsObj.GetUnitNonSeaWeight(unitSelected, false))))) + " / " + Strings.Trim(Conversion.Str( airCarryCapPts));
             let mut local24: &Graphics = &Expression;
            trect1 = Rectangle::new(x5, 115, width1, 14);
            let mut rect1_22: &Rectangle = &trect1
            trect2 = Rectangle::new(x5, 129, width1, 23);
            let mut rect2_22: &Rectangle = &trect2
            txt2_22: String = str17;
            DrawMod.MakeFullBoxVic2( local24, rect1_22, "AIRCARRIER", rect2_22, txt2_22);
            let mut x6: i32 =  x5 + 100;
            let mut width2: i32 =  80;
            str18: String = Strings.Trim(Conversion.Str(Operators.AddObject(this.game.HandyFunctionsObj.GetUnitNonSeaWeight(unitSelected, true), this.game.HandyFunctionsObj.GetUnitExcessWeight(unitSelected))));
             let mut local25: &Graphics = &Expression;
            trect1 = Rectangle::new(x6, 25, width2, 14);
            let mut rect1_23: &Rectangle = &trect1
            trect2 = Rectangle::new(x6, 39, width2, 23);
            let mut rect2_23: &Rectangle = &trect2
            txt2_23: String = str18;
            DrawMod.MakeFullBoxVic2( local25, rect1_23, "WEIGHT", rect2_23, txt2_23);
            tname = Strings.Trim(Conversion.Str( this.game.HandyFunctionsObj.GetUnitStackPts(unitSelected)));
             let mut local26: &Graphics = &Expression;
            trect1 = Rectangle::new(x6, 70, width2, 14);
            let mut rect1_24: &Rectangle = &trect1
            trect2 = Rectangle::new(x6, 84, width2, 23);
            let mut rect2_24: &Rectangle = &trect2
            txt2_24: String = tname;
            DrawMod.MakeFullBoxVic2( local26, rect1_24, "STACK", rect2_24, txt2_24);
            if (!this.game.Data.UnitObj[unitSelected].IsHQ)
            {
              let mut x7: i32 =  x6 + 100;
              let mut width3: i32 =  80;
              let mut hq: i32 =  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].HQ;
              let mut num15: i32 =  this.game.HandyFunctionsObj.GetStaffPercent(hq, true);
              let mut num16: i32 =  this.game.HandyFunctionsObj.GetStaffPercent(hq, true);
              let mut num17: i32 =  this.game.HandyFunctionsObj.GetStaffPercent(hq);
              let mut num18: i32 =  this.game.HandyFunctionsObj.GetStaffPercent(hq);
              if (num15 > 100)
                num15 = 100;
              if (num16 > 100)
                num16 = 100;
              if (num17 > 100)
                num17 = 100;
              if (num18 > 100)
                num18 = 100;
              let mut Number1: i32 =   Math.Round(  Math.Round( num15 *  this.game.HandyFunctionsObj.GetStaffCombatMod(hq) * ( this.game.HandyFunctionsObj.Gethqpow(this.game.EditObj.UnitSelected) / 100.0)) +  num17 *  this.game.Data.RuleVar[140] * ( this.game.HandyFunctionsObj.Gethqpow(this.game.EditObj.UnitSelected) / 100.0));
              let mut Number2: i32 =   Math.Round(  Math.Round( num16 *  this.game.HandyFunctionsObj.GetStaffMoraleMod(hq) * ( this.game.HandyFunctionsObj.Gethqpow(this.game.EditObj.UnitSelected) / 100.0)) +  num18 *  this.game.Data.RuleVar[141] * ( this.game.HandyFunctionsObj.Gethqpow(this.game.EditObj.UnitSelected) / 100.0));
              str19: String = Strings.Trim(Conversion.Str( Number1)) + "%";
               let mut local27: &Graphics = &Expression;
              trect1 = Rectangle::new(x7, 25, width3, 14);
              let mut rect1_25: &Rectangle = &trect1
              trect2 = Rectangle::new(x7, 39, width3, 23);
              let mut rect2_25: &Rectangle = &trect2
              txt2_25: String = str19;
              DrawMod.MakeFullBoxVic2( local27, rect1_25, "STF.COMBAT", rect2_25, txt2_25);
              tname = Strings.Trim(Conversion.Str( Number2)) + "%";
               let mut local28: &Graphics = &Expression;
              trect1 = Rectangle::new(x7, 70, width3, 14);
              let mut rect1_26: &Rectangle = &trect1
              trect2 = Rectangle::new(x7, 84, width3, 23);
              let mut rect2_26: &Rectangle = &trect2
              txt2_26: String = tname;
              DrawMod.MakeFullBoxVic2( local28, rect1_26, "STF.MORALE", rect2_26, txt2_26);
            }
          }
        }
      }
      if (false)
      {
        let mut selectX: i32 =  this.game.SelectX;
        let mut selectY: i32 =  this.game.SelectY;
        Expression = Graphics.FromImage((Image) this.OwnBitmap);
        let mut landscapeType: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[selectX, selectY].LandscapeType;
        let mut spriteNr: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[selectX, selectY].SpriteNr;
        let mut num19: i32 =   Math.Round( this.game.ScreenWidth / 2.0 - 174.0);
        let mut y1: i32 =  55;
        if (this.game.Data.Round > 0 && this.game.Data.ShrowdOn & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[selectX, selectY].get_SeeNow(this.game.Data.Turn) < 1)
        {
          DrawMod.DrawBlock( Expression, num19, y1, 384, 144, 0, 0, 0,  byte.MaxValue);
          tstring: String = tname + "Shrouded (" + Conversion.Str( selectX) + "," + Conversion.Str( selectY) + ")";
          DrawMod.DrawText( Expression, tstring, Font::new("Times New Roman", 12f, FontStyle.Bold, GraphicsUnit.Pixel), 10, 35);
        }
        if (landscapeType > -1 & spriteNr > -1)
        {
          let mut nr1: i32 =  this.game.Data.LandscapeTypeObj[landscapeType].BasicPicID[spriteNr];
           let mut local29: &Graphics = &Expression;
          bitmap = BitmapStore.GetBitmap(nr1);
           let mut local30: &Bitmap = &bitmap;
          let mut x8: i32 =  num19;
          let mut y2: i32 =  y1;
          DrawMod.DrawScaled( local29,  local30, x8, y2, 384, 144);
          if (this.game.Data.MapObj[0].HexObj[selectX, selectY].Location > -1 && this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[selectX, selectY].Location].Type].PictureLT > -1)
          {
            let mut nr2: i32 =  this.game.Data.LandscapeTypeObj[this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[selectX, selectY].Location].Type].PictureLT].BasicPicID[this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[selectX, selectY].Location].Type].PictureSprite];
             let mut local31: &Graphics = &Expression;
            bitmap = BitmapStore.GetBitmap(nr2);
             let mut local32: &Bitmap = &bitmap;
            let mut x9: i32 =  num19;
            let mut y3: i32 =  y1;
            DrawMod.DrawScaled( local31,  local32, x9, y3, 384, 144);
          }
        }
        DrawMod.DrawRectangle( Expression, num19, y1, 384, 144,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
        str: String = !(this.game.Data.ShrowdOn & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[selectX, selectY].get_SeeNow(this.game.Data.Turn) == 0) ? (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[selectX, selectY].Regime != -1 ? this.game.Data.RegimeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[selectX, selectY].Regime].Name : "Neutral") : "Unknown".to_owned();
        let mut y: i32 =  15;
         let mut local: &Graphics = &Expression;
        trect1 = Rectangle::new(num19, y, 384, 14);
        let mut rect1: &Rectangle = &trect1
        trect2 = Rectangle::new(num19, y + 14, 384, 23);
        let mut rect2: &Rectangle = &trect2
        txt2: String = str;
        DrawMod.MakeFullBoxVic2( local, rect1, "HEX OWNER", rect2, txt2);
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
      Expression = (Graphics) null;
    }

    pub fn PopUpRefresh()
    {
      this.game.EditObj.HandCard = -1;
      this.DoRefresh();
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.game.EditObj.BattleTimerActive)
        return windowReturnClass;
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (this.MouseData[index] > 0 && x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
        {
          if (this.MouseData[index] == 1)
          {
            this.game.EditObj.SetViewMode2 = 0;
            this.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (this.MouseData[index] == 2)
          {
            this.game.EditObj.SetViewMode2 = 1;
            this.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (this.MouseData[index] == 3)
          {
            this.game.EditObj.SetViewMode2 = 2;
            this.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
      }
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 =  this.SubPartID[index1];
            if (num1 == this.OfficerId)
            {
              let mut num2: i32 =  x - this.SubPartX[index1];
              let mut num3: i32 =  y - this.SubPartY[index1];
              if (num2 > 140 & num2 < 340 & num3 > 18 & num3 < 36 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn)
              {
                str: String = Interaction.InputBox("Give New Name for commander", "Rename", this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].CommanderName);
                if (Strings.Len(str) > 33)
                  str = Strings.Left(str, 33);
                if (Operators.CompareString(Strings.Trim(str), "", false) != 0)
                  this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].CommanderName = str;
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime, this.game.Data.Turn) | !this.game.Data.FOWOn | this.game.Data.Round == 0)
              {
                index2: i32;
                index3: i32;
                if (num2 > 50 & num3 > 107 & num2 < 350 & num3 < 177)
                {
                  this.game.EditObj.PopupValue = 4;
                  this.game.EditObj.HandCard = this.game.Data.HistoricalUnitObj[index2].HandCard[index3];
                  windowReturnClass.AddCommand(5, 10);
                  this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num2 > 50 & num3 > 7 & num2 < 138 & num3 < 103)
                {
                  this.game.EditObj.PopupValue = 4;
                  this.game.EditObj.HandCard = this.game.Data.HistoricalUnitObj[index2].HandCard[index3];
                  windowReturnClass.AddCommand(5, 10);
                  this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn)
                {
                  let mut historical: i32 =  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical;
                  if (historical > -1)
                  {
                    let mut num4: i32 =  Math.Min(3, this.game.Data.HistoricalUnitObj[historical].HandCardCounter);
                    for (let mut index4: i32 =  0; index4 <= num4; index4 += 1)
                    {
                      if (num2 > 140 + index4 * 45 & num3 > 45 & num2 < 185 + index4 * 45 & num3 < 105)
                      {
                        this.game.EditObj.PopupValue = 2;
                        this.game.EditObj.HandCard = this.game.Data.HistoricalUnitObj[historical].HandCard[index4];
                        windowReturnClass.AddCommand(5, 10);
                        this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                    }
                  }
                }
              }
            }
            else if (num1 == this.UnitHeaderId)
            {
              if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn | this.game.Data.Round == 0)
              {
                let mut num5: i32 =  x - this.SubPartX[index1];
                let mut num6: i32 =  y - this.SubPartY[index1];
                if (num5 > 125 & num5 < 277 & num6 > 70 & num6 < 88)
                {
                  str: String = Interaction.InputBox("Give New Name for Unit", "Rename", this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Name);
                  if (Strings.Len(str) > 25)
                    str = Strings.Left(str, 25);
                  if (Operators.CompareString(Strings.Trim(str), "", false) != 0)
                    this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Name = str;
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num5 > 125 & num5 < 277 & num6 > 90 & num6 < 108 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn | this.game.Data.Round == 0)
                {
                  let mut hq: i32 =  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].HQ;
                  if (hq > -1 & this.game.EditObj.OrderType == 0)
                  {
                    if (this.game.Data.UnitObj[hq].X > -1 & this.game.Data.UnitObj[hq].Y > -1)
                    {
                      this.game.SelectX = this.game.Data.UnitObj[hq].X;
                      this.game.SelectY = this.game.Data.UnitObj[hq].Y;
                      this.game.EditObj.UnitSelected = hq;
                    }
                    else
                    {
                      this.game.SelectX = this.game.Data.UnitObj[this.game.Data.UnitObj[hq].OnBoard].X;
                      this.game.SelectY = this.game.Data.UnitObj[this.game.Data.UnitObj[hq].OnBoard].Y;
                      this.game.EditObj.UnitSelected = this.game.Data.UnitObj[hq].OnBoard;
                    }
                    this.game.HandyFunctionsObj.SetcornerXY2();
                    this.game.EditObj.TempCoordList = CoordList::new();
                    windowReturnClass.AddCommand(4, 12);
                    windowReturnClass.AddCommand(4, 18);
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                }
                if (num5 > 44 & num5 < 122 & num6 > 60 & num6 < 138 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
                {
                  ColorDialog colorDialog = ColorDialog::new();
                  colorDialog.Color = Color.FromArgb( byte.MaxValue, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Red, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Green, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Blue);
                  if (colorDialog.ShowDialog() == DialogResult.OK)
                  {
                    this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Blue =  colorDialog.Color.B;
                    UnitClass unitClass1 = this.game.Data.UnitObj[this.game.EditObj.UnitSelected];
                    color: Color = colorDialog.Color;
                    let mut g: i32 =   color.G;
                    unitClass1.Green = g;
                    UnitClass unitClass2 = this.game.Data.UnitObj[this.game.EditObj.UnitSelected];
                    color = colorDialog.Color;
                    let mut r: i32 =   color.R;
                    unitClass2.Red = r;
                  }
                  this.dostuff();
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
              }
            }
            else
            {
              if (num1 == this.OptionsListId)
              {
                let mut num7: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                if (num7 > -1 & this.game.EditObj.OrderType == 0)
                {
                  this.game.EditObj.SFSelected = num7;
                  if (this.game.Data.Round > 0)
                  {
                    this.game.EditObj.PopupValue = 6;
                    this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                    windowReturnClass.AddCommand(5, 10);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                }
                else
                  this.SubPartFlag[index1] = true;
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.UnitsfId)
              {
                let mut sfSelected: i32 =  this.game.EditObj.SFSelected;
                this.game.EditObj.SFSelected = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                if (this.game.EditObj.SFSelected > -1 & this.game.EditObj.OrderType == 0)
                {
                  windowReturnClass.AddCommand(4, 44);
                  if (sfSelected == this.game.EditObj.SFSelected & this.game.Data.Round > 0)
                  {
                    this.game.EditObj.PopupValue = 6;
                    this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                    windowReturnClass.AddCommand(5, 10);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (this.game.EditObj.SFSelected <= this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SFCount & this.game.Data.Round > 0)
                  {
                    this.game.EditObj.PopupValue = 6;
                    this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                    windowReturnClass.AddCommand(5, 10);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
              }
              else
              {
                if (num1 == this.OptionsList2Id)
                {
                  let mut num8: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                  if (b != 1)
                    this.detail2 = -1;
                  if (this.CurrentView == 2)
                  {
                    if (num8 > -1)
                      this.detail2 = num8;
                    this.dostuff();
                  }
                  this.SubPartFlag[index1] = true;
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.DefendPercentID)
                {
                  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SODefendPercent = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1], b);
                  this.SubPartFlag[index1] = true;
                  windowReturnClass.SetFlag(true);
                }
                else if (num1 == this.RequestSupID)
                {
                  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOSupReqPercent = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1], b);
                  this.SubPartFlag[index1] = true;
                  windowReturnClass.SetFlag(true);
                }
                else if (num1 == this.SOB1Id)
                {
                  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOInterceptRdnStop = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1], b);
                  this.SubPartFlag[index1] = true;
                  windowReturnClass.SetFlag(true);
                }
                else if (num1 == this.AsId)
                {
                  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SODoAS = !this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SODoAS;
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                }
                else if (num1 == this.DefId)
                {
                  let mut unitSelected: i32 =  this.game.EditObj.UnitSelected;
                  if (this.game.Data.UnitObj[unitSelected].SODefendPercent == 0)
                    this.game.Data.UnitObj[unitSelected].SODefendPercent = 25;
                  else if (this.game.Data.UnitObj[unitSelected].SODefendPercent == 25)
                    this.game.Data.UnitObj[unitSelected].SODefendPercent = 50;
                  else if (this.game.Data.UnitObj[unitSelected].SODefendPercent == 50)
                    this.game.Data.UnitObj[unitSelected].SODefendPercent = 75;
                  else
                    this.game.Data.UnitObj[unitSelected].SODefendPercent = 0;
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                }
                else if (num1 == this.DitchId)
                {
                  str: String = Conversions.ToString(Conversion.Val(Interaction.InputBox("Give amount of supply points to ditch", "Shadow Empire : Planetary Conquest")));
                  if (Operators.CompareString(Strings.Trim(str), "", false) != 0)
                  {
                    let mut num9: i32 =  Conversions.ToInteger(str);
                    if (num9 < 1)
                      num9 = 0;
                    if (num9 > this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Supply)
                      num9 = this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Supply;
                    UnitClass[] unitObj = this.game.Data.UnitObj;
                    UnitClass[] unitClassArray = unitObj;
                    let mut unitSelected: i32 =  this.game.EditObj.UnitSelected;
                    let mut index5: i32 =  unitSelected;
                    unitClassArray[index5].Supply = unitObj[unitSelected].Supply - num9;
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                }
                else if (num1 == this.ReserveId)
                {
                  str: String = Conversions.ToString(Conversion.Val(Interaction.InputBox("Give desired reserve in supply points", "Shadow Empire : Planetary Conquest")));
                  if (Operators.CompareString(Strings.Trim(str), "", false) != 0)
                  {
                    let mut num10: i32 =  Conversions.ToInteger(str);
                    if (num10 < 1)
                      num10 = 0;
                    if (num10 > 99999)
                      num10 = 99999;
                    this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Reserve = num10;
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                }
                else if (num1 == this.SupplyId)
                {
                  let mut unitSelected: i32 =  this.game.EditObj.UnitSelected;
                  if (this.game.Data.UnitObj[unitSelected].SOSupReqPercent == 50)
                    this.game.Data.UnitObj[unitSelected].SOSupReqPercent = 75;
                  else if (this.game.Data.UnitObj[unitSelected].SOSupReqPercent == 75)
                    this.game.Data.UnitObj[unitSelected].SOSupReqPercent = 100;
                  else
                    this.game.Data.UnitObj[unitSelected].SOSupReqPercent = 50;
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                }
                else if (num1 == this.ReplacementID)
                {
                  if (this.game.Data.Round == 0 | this.game.Data.Turn == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime)
                  {
                    if (!this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
                    {
                      if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent == 0)
                        this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent = 25;
                      else if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent == 25)
                        this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent = 50;
                      else if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent == 50)
                        this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent = 75;
                      else if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent == 75)
                        this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent = 100;
                      else
                        this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent = 0;
                    }
                    else if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent == 0)
                      this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent = 25;
                    else if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent == 25)
                      this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent = 50;
                    else if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent == 50)
                      this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent = 75;
                    else if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent == 75)
                      this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent = 100;
                    else
                      this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOReplacementPercent = 0;
                  }
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                }
                else if (num1 == this.InterceptId)
                {
                  if (this.game.Data.Round == 0 | this.game.Data.Turn == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime && this.game.HandyFunctionsObj.HasUnitAirSF(this.game.EditObj.UnitSelected))
                  {
                    if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOInterceptRdnStop == 25)
                      this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOInterceptRdnStop = 100;
                    else if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOInterceptRdnStop == 50)
                      this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOInterceptRdnStop = 25;
                    else if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOInterceptRdnStop == 75)
                      this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOInterceptRdnStop = 50;
                    else
                      this.game.Data.UnitObj[this.game.EditObj.UnitSelected].SOInterceptRdnStop = 75;
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                  }
                }
                else if (num1 == this.ClearId)
                {
                  if (this.game.EditObj.OrderType == 3)
                  {
                    if (this.game.ProcessingObj.SetUnitHq(this.game.EditObj.OrderUnit, -1).OK)
                    {
                      this.game.EditObj.OrderType = 0;
                      windowReturnClass.AddCommand(4, 12);
                      windowReturnClass.AddCommand(4, 44);
                      windowReturnClass.AddCommand(4, 29);
                      this.game.SelectX = this.game.EditObj.OrderX;
                      this.game.SelectY = this.game.EditObj.OrderY;
                      this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                    }
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                  }
                }
                else if (num1 == this.OrderOk2Id)
                {
                  let mut orderType1: i32 =  this.game.EditObj.OrderType;
                }
                else if (num1 == this.OrderOkId)
                {
                  let mut orderType2: i32 =  this.game.EditObj.OrderType;
                }
                else if (num1 == this.SliderButtonId)
                {
                  if (this.game.EditObj.OrderType == 5)
                  {
                    this.game.EditObj.orderInt = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                    this.SubPartFlag[index1] = true;
                    windowReturnClass.SetFlag(true);
                  }
                }
                else if (num1 == this.OrderUpId)
                {
                  let mut orderType3: i32 =  this.game.EditObj.OrderType;
                }
                else if (num1 == this.OrderDownId)
                {
                  let mut orderType4: i32 =  this.game.EditObj.OrderType;
                }
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
