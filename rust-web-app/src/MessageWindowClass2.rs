// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MessageWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class MessageWindowClass2 : WindowClass
  {
     okid: i32;
     tbacknr: i32;
     oktextid: i32;
     noteid: i32;
     note2id: i32;
     cloudid: i32;
     Pic1Id: i32;
     TAid: i32;
     FromMessage: i32;

    pub MessageWindowClass2( tGame: GameClass)
      : base( tGame, 680, 480, 8)
    {
      this.FromMessage = tGame.EditObj.FromMessage;
      this.ViewMessage();
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
            this.game.EditObj.TipButton = false;
            this.SubPartList[index].HandleToolTip(x - this.SubPartX[index], y - this.SubPartY[index]);
            if (this.game.EditObj.TipButton)
              return;
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

    pub fn ViewMessage()
    {
      if (this.okid > 0)
      {
        this.RemoveSubPart(this.okid);
        this.okid = 0;
      }
      if (this.TAid > 0)
      {
        this.RemoveSubPart(this.TAid);
        this.TAid = 0;
      }
      this.NewBackGroundAndClearAll(680, 480, -1);
      Graphics g = Graphics.FromImage((Image) this.OwnBitmap);
      DrawMod.DrawMessFrame( this.OwnBitmap,  g, 0, 0, 680, 480);
      this.BackBitmap = (Bitmap) this.OwnBitmap.Clone();
      if (this.FromMessage == -1)
        return;
      if (Strings.Len(this.game.Data.RegimeObj[this.game.Data.Turn].MessWav[this.FromMessage]) > 0)
      {
        SoundMod.STopEventWave();
        SoundMod.PlayEventWave(this.game.AppPath + "sound/" + this.game.Data.RegimeObj[this.game.Data.Turn].MessWav[this.FromMessage],  this.game.EditObj);
      }
      width1: i32;
      height1: i32;
      num1: i32;
      if (this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[this.FromMessage] > -1)
      {
        let mut index: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[this.FromMessage];
        let mut nr: i32 =  index < 10000 ? this.game.Data.EventPicNr[index] : this.game.Data.HistoricalUnitObj[index - 10000].CommanderSpriteID;
        Rectangle rectangle;
        num2: i32;
        if (nr > -1)
        {
          width1 = BitmapStore.GetWidth(nr);
          height1 = BitmapStore.Getheight(nr);
          if (width1 > 340)
          {
            height1 =  Math.Round( height1 * (340.0 /  width1));
            width1 =  Math.Round( width1 * (340.0 /  width1));
          }
          if (height1 > 150)
          {
            width1 =  Math.Round( width1 * (150.0 /  height1));
            height1 =  Math.Round( height1 * (150.0 /  height1));
          }
          rectangle = Rectangle::new( Math.Round(180.0 -  width1 / 2.0), 15, width1, height1);
          num2 = height1 + 15;
        }
        if (!Information.IsNothing( this.game.Data.RegimeObj[this.game.Data.Turn].MesNote2[this.FromMessage]) && this.game.Data.RegimeObj[this.game.Data.Turn].MesNote2[this.FromMessage].Length > 0)
          rectangle.X = 15;
        rectangle.X += 40;
        rectangle.Y += 30;
        num1 = num2 + 30;
        if (this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[this.FromMessage] > 10000)
        {
          let mut his: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[this.FromMessage] - 10000;
          DrawMod.DrawOfficer(g, his, rectangle.X, rectangle.Y, rectangle.Width, rectangle.Height);
        }
        else
        {
           let mut local1: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(nr);
           let mut local2: &Bitmap = &bitmap;
          let mut x: i32 =  rectangle.X;
          let mut y: i32 =  rectangle.Y;
          let mut width2: i32 =  rectangle.Width;
          let mut height2: i32 =  rectangle.Height;
          DrawMod.DrawScaled( local1,  local2, x, y, width2, height2);
          DrawMod.DrawFrame( this.OwnBitmap,  this.BackBitmap,  g, rectangle.X, rectangle.Y, rectangle.Width + 1, rectangle.Height + 1, -1, -1);
        }
      }
      else
        num1 = 15 + 30;
      SizeF sizeF1 = SizeF::new();
      if (!Information.IsNothing( this.game.Data.RegimeObj[this.game.Data.Turn].MesNote2[this.FromMessage]) && this.game.Data.RegimeObj[this.game.Data.Turn].MesNote2[this.FromMessage].Length > 0 & width1 < 150)
      {
        str: String = this.game.Data.RegimeObj[this.game.Data.Turn].MesNote2[this.FromMessage];
        num3: i32;
        for (SizeF sizeF2 = g.MeasureString(str, this.game.MarcFont7);  sizeF2.Width >  (620 - width1); sizeF2 = g.MeasureString(str, this.game.MarcFont7))
        {
          str = Strings.Left(str, Strings.Len(str) - 1);
          num3 = 1;
        }
        if (num3 == 1)
          str = Strings.Left(str, Strings.Len(str) - 1) + "...";
        DrawMod.DrawTextColouredMarc( g, str, this.game.MarcFont7, 55 + width1 + 15,  Math.Round(30.0 +  height1 / 4.0), Color.White);
      }
      let mut trows: i32 =   Math.Round(Conversion.Int( (370 - num1) / 16.0));
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(this.game, 569, trows, this.game.MarcFont8, this.game.Data.RegimeObj[this.game.Data.Turn].MessString[this.FromMessage], tbackbitmap: ( this.BackBitmap), bbx: 50, bby: num1, tUseEncy: true);
      this.TAid = this.AddSubPart( tsubpart1, 50, num1, 569, 16 * (trows + 2), 0);
      let mut num4: i32 =  16 * (trows + 1);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("PRESS ANY KEY", 150, "Click to indicate you have read message.\r\nOr press any key instead.",  this.OwnBitmap, 270, num1 + num4 + 36, theight: 30, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
      this.okid = this.AddSubPart( tsubpart2, 270, num1 + num4 + 46, 150, 30, 1);
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (nr == 40)
      {
        this.SubPartList[this.SubpartNr(this.TAid)].ShiftDown();
        this.SubPartFlag[this.SubpartNr(this.TAid)] = true;
        windowReturnClass.SetFlag(true);
      }
      if (nr == 38)
      {
        this.SubPartList[this.SubpartNr(this.TAid)].ShiftUp();
        this.SubPartFlag[this.SubpartNr(this.TAid)] = true;
        windowReturnClass.SetFlag(true);
      }
      if (nr == 37)
      {
        this.SubPartList[this.SubpartNr(this.TAid)].ShiftLeft();
        this.SubPartFlag[this.SubpartNr(this.TAid)] = true;
        windowReturnClass.SetFlag(true);
      }
      if (nr == 39)
      {
        this.SubPartList[this.SubpartNr(this.TAid)].ShiftRight();
        this.SubPartFlag[this.SubpartNr(this.TAid)] = true;
        windowReturnClass.SetFlag(true);
      }
      return windowReturnClass;
    }

    pub HandleKeyup: WindowReturnClass(nr: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        switch (nr)
        {
          case 37:
            return windowReturnClass;
          case 38:
            return windowReturnClass;
          case 39:
            return windowReturnClass;
          case 40:
            return windowReturnClass;
          default:
            if (nr > 0)
            {
              windowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.okid)] + 1, this.SubPartY[this.SubpartNr(this.okid)] + 1, 1);
              windowReturnClass.SetFlag(true);
              break;
            }
            break;
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
            let mut num: i32 =  this.SubPartID[index];
            if (num == this.TAid)
            {
              this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == this.okid)
            {
              if (this.FromMessage >= this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter)
              {
                SoundMod.STopEventWave();
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              this += 1.FromMessage;
              this.ViewMessage();
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
