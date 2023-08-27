// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.HandCardWindowClass2
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
  pub class HandCardWindowClass2 : WindowClass
  {
     okid: i32;
     cancelid: i32;
     oktextid: i32;
     Pic1Id: i32;
     TAid: i32;
     His: i32;
     Card: i32;
     Unr: i32;
     tSelectX: i32;
     tSelectY: i32;
     tSelectMap: i32;
     tCornerX: i32;
     tCornerY: i32;

    pub HandCardWindowClass2( tGame: GameClass)
      : base( tGame, 380, 380, 8)
    {
      this.His = tGame.Data.UnitObj[tGame.EditObj.UnitSelected].Historical;
      this.Unr = tGame.EditObj.UnitSelected;
      this.Card = tGame.EditObj.HandCard;
      this.tSelectX = this.game.SelectX;
      this.tSelectY = this.game.SelectY;
      this.tCornerX = this.game.CornerX;
      this.tCornerY = this.game.CornerY;
      this.tSelectMap = this.game.EditObj.MapSelected;
      this.ViewCard();
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            this.SubPartList[index].DescriptInfo(x - this.SubPartX[index], y - this.SubPartY[index]);
            if (Operators.CompareString(this.SubPartList[index].Descript, "", false) > 0)
            {
              this.game.EditObj.TipButton = true;
              this.game.EditObj.TipTitle = "";
              this.game.EditObj.TipText = this.SubPartList[index].Descript;
              return;
            }
          }
        }
      }
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
        {
          if (this.MouseData[index] > 0)
            this.game.EditObj.TipButton = true;
          this.game.EditObj.TipTitle = this.MouseTitle[index];
          this.game.EditObj.TipText = this.MouseText[index];
          break;
        }
      }
    }

    pub fn ViewCard()
    {
      this.ClearMouse();
      this.NewBackGroundAndClearAll(380, 380, -1);
      Graphics g = Graphics.FromImage((Image) this.OwnBitmap);
      DrawMod.DrawMessFrame( this.OwnBitmap,  g, 0, 0, 380, 380);
      this.BackBitmap = (Bitmap) this.OwnBitmap.Clone();
      if (this.okid > 0)
      {
        this.RemoveSubPart(this.okid);
        this.okid = 0;
      }
      if (this.cancelid > 0)
      {
        this.RemoveSubPart(this.cancelid);
        this.cancelid = 0;
      }
       let mut local1: &Graphics = &g;
      bitmap: Bitmap = this.game.CustomBitmapObj.DrawActionCardMarc2(this.game.Data.Turn, this.Card);
       let mut local2: &Bitmap = &bitmap;
      DrawMod.DrawSimple( local1,  local2, 95, 30);
      if (this.game.Data.ActionCardObj[this.Card].MouseOver.Length > 0)
      {
        Rectangle trect = Rectangle::new(95, 30, 190, 266);
        this.AddMouse( trect, "", this.game.Data.ActionCardObj[this.Card].MouseOver);
      }
      if ((this.game.Data.RegimeObj[this.game.Data.Turn].ResPts >= this.game.Data.ActionCardObj[this.Card].PPCost | this.game.Data.ActionCardObj[this.Card].PPCost == 0) & (this.game.Data.ActionCardObj[this.Card].HisVarCostType == -1 | this.game.Data.HistoricalUnitObj[this.His].GiveHisVarValue(this.game.Data.ActionCardObj[this.Card].HisVarCostType) >= this.game.Data.ActionCardObj[this.Card].HisVarCostQty))
      {
        let mut tsubpart1: SubPartClass =  new TextButtonPartClass("PLAY CARD", 125, "Click to play this actioncard. [SPACE]",  this.OwnBitmap, 50, 310, theight: 40, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.okid = this.AddSubPart( tsubpart1, 50, 310, 105, 40, 1);
        let mut tsubpart2: SubPartClass =  new TextButtonPartClass("CANCEL", 105, "Click to not play this card. [ESC]",  this.OwnBitmap, 205, 310, theight: 40, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.cancelid = this.AddSubPart( tsubpart2, 205, 310, 105, 40, 1);
      }
      else
      {
        let mut tsubpart3: SubPartClass =  new TextButtonPartClass("PLAY CARD", 125, "You do not have the PP and/or commander points needed.",  this.OwnBitmap, 50, 310, true, theight: 40, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.oktextid = this.AddSubPart( tsubpart3, 50, 310, 105, 40, 1);
        let mut tsubpart4: SubPartClass =  new TextButtonPartClass("CANCEL", 105, "Click to not play this card. [ESC]",  this.OwnBitmap, 205, 310, theight: 40, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.cancelid = this.AddSubPart( tsubpart4, 205, 310, 105, 40, 1);
      }
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27)
        {
          windowReturnClass.AddCommand(6, 0);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        if (nr == 32)
        {
          windowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.okid)] + 1, this.SubPartY[this.SubpartNr(this.okid)] + 1, 1);
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
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 =  this.SubPartID[index1];
            if (num1 == this.okid)
            {
              this.game.EditObj.SkippedPreSelectPopup = false;
              if (this.game.Data.ActionCardObj[this.Card].AreaSlot > -1)
              {
                this.game.ProcessingObj.PlayCardPreEvent(this.Card);
                if (this.game.HandyFunctionsObj.CardSelectHexTestPossible(this.Card, this.game.Data.ActionCardObj[this.Card].AreaSlot, this.game.Data.ActionCardObj[this.Card].AreaValue))
                {
                  this.game.EditObj.DoCardSlot = this.Card;
                  this.game.EditObj.AreaSlot = this.game.Data.ActionCardObj[this.Card].AreaSlot;
                  this.game.EditObj.AreaValue = this.game.Data.ActionCardObj[this.Card].AreaValue;
                  this.game.EditObj.PopupValue = 1;
                  windowReturnClass.AddCommand(5, 14);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                this.game.EditObj.SkippedPreSelectPopup = true;
              }
              else if (this.game.Data.ActionCardObj[this.Card].UnitSelect)
              {
                this.game.ProcessingObj.PlayCardPreEvent(this.Card);
                if (this.game.HandyFunctionsObj.CardSelectUnitTestPossible(this.Card))
                {
                  this.game.EditObj.PopupValue = 3;
                  windowReturnClass.AddCommand(5, 14);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                this.game.EditObj.SkippedPreSelectPopup = true;
              }
              let mut messCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
              this.game.ProcessingObj.PlayCardByUnit(this.game.EditObj.UnitSelected, this.Card);
              this.game.EditObj.AreaX = -1;
              this.game.EditObj.AreaSlot = -1;
              this.game.EditObj.HandCard = -1;
              if (Strings.Len(this.game.Data.LoadGame) > 0)
              {
                this.game.FormRef.Cursor = Cursors.WaitCursor;
                Form formRef =  this.game.FormRef;
                this.game.HandyFunctionsObj.LoadGameNow();
                this.game.FormRef = (Form1) formRef;
                this.game.FormRef.Cursor = Cursors.Default;
                windowReturnClass.AddCommand(3, 13);
                return windowReturnClass;
              }
              let mut locCounter: i32 =  this.game.Data.LocCounter;
              Number: i32;
              for (let mut locnr: i32 =  0; locnr <= locCounter; locnr += 1)
              {
                if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.LocObj[locnr].X, this.game.Data.LocObj[locnr].Y].Regime == this.game.Data.Turn)
                {
                  let mut index2: i32 =  0;
                  do
                  {
                    if (this.game.Data.LocObj[locnr].Production[index2] > -1 && !this.game.HandyFunctionsObj.CanProduceItem(locnr, this.game.Data.Turn, this.game.Data.LocObj[locnr].Production[index2]).result)
                    {
                      Number += 1;
                      this.game.Data.LocObj[locnr].Production[index2] = -1;
                      this.game.Data.LocObj[locnr].ProdPointRemainder[index2] = 0;
                      this.game.Data.LocObj[locnr].ProdPercent[index2] = 0;
                    }
                    index2 += 1;
                  }
                  while (index2 <= 3);
                }
              }
              if (Number > 0)
              {
                let mut num2: i32 =   Interaction.MsgBox( (Conversion.Str( Number) + " production lines have been cancelled due to this action card being played."), Title: ( "Shadow Empire : Planetary Conquest"));
              }
              this.game.CornerX = this.tCornerX;
              this.game.CornerY = this.tCornerY;
              this.game.SelectX = this.tSelectX;
              this.game.SelectY = this.tSelectY;
              this.game.EditObj.MapSelected = this.tSelectMap;
              this.game.EditObj.TempCoordList = CoordList::new();
              this.game.HandyFunctionsObj.RedimTempValue(9999);
              if ( this.game.Data.RuleVar[701] > 0.0 & this.game.Data.Product >= 6)
                this.game.EditObj.udsReturnFromPopup = true;
              if (this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter > messCounter)
              {
                this.game.EditObj.PopupValue = 0;
                this.game.EditObj.FromMessage = this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
                windowReturnClass.AddCommand(5, 14);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              this.game.EditObj.DoCardSlot = -1;
              return windowReturnClass;
            }
            if (num1 == this.cancelid)
            {
              this.game.CornerX = this.tCornerX;
              this.game.CornerY = this.tCornerY;
              this.game.SelectX = this.tSelectX;
              this.game.SelectY = this.tSelectY;
              this.game.EditObj.MapSelected = this.tSelectMap;
              this.game.EditObj.TempCoordList = CoordList::new();
              this.game.HandyFunctionsObj.RedimTempValue(9999);
              windowReturnClass.AddCommand(6, 0);
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
