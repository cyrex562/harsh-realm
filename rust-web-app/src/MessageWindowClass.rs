// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MessageWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class MessageWindowClass : WindowClass
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

    pub MessageWindowClass( tGame: GameClass)
      : base( tGame, 810, 610, BackSprite: tGame.BACKGROUND2MARC, tBackSpriteScaled: true)
    {
      self.tbacknr = tGame.BACKGROUND2MARC;
      self.FromMessage = tGame.EditObj.FromMessage;
      self.ViewMessage();
    }

    pub MessageWindowClass( tGame: GameClass, TempShit: i32)
      : base( tGame, 810, 610, 7, tGame.BACKGROUND2MARC, true)
    {
      self.tbacknr = tGame.BACKGROUND2MARC;
      self.FromMessage = tGame.EditObj.FromMessage;
      self.ViewMessage();
    }

    pub fn ViewMessage()
    {
      if (self.Pic1Id > 0)
        self.RemoveSubPart(self.Pic1Id);
      if (self.okid > 0)
      {
        self.RemoveSubPart(self.okid);
        self.okid = 0;
      }
      if (self.oktextid > 0)
        self.RemoveSubPart(self.oktextid);
      if (self.TAid > 0)
        self.RemoveSubPart(self.TAid);
      if (self.noteid > 0)
        self.RemoveSubPart(self.noteid);
      if (self.note2id > 0)
        self.RemoveSubPart(self.note2id);
      if (self.cloudid > 0)
        self.RemoveSubPart(self.cloudid);
      if (self.game.Data.RegimeObj[self.game.Data.Turn].MesStyle[self.FromMessage] == 1)
        self.NewBackGroundAndClearAll(810, 610, self.game.BACKGROUND2MARC);
      Graphics g = Graphics.FromImage((Image) self.OwnBitmap);
      if (Strings.Len(self.game.Data.RegimeObj[self.game.Data.Turn].MessWav[self.FromMessage]) > 0)
      {
        SoundMod.STopEventWave();
        SoundMod.PlayEventWave(self.game.AppPath + "sound/" + self.game.Data.RegimeObj[self.game.Data.Turn].MessWav[self.FromMessage],  self.game.EditObj);
      }
      num1: i32;
      if (self.game.Data.RegimeObj[self.game.Data.Turn].MessFrontPic[self.FromMessage] > -1)
      {
        let mut index: i32 =  self.game.Data.RegimeObj[self.game.Data.Turn].MessFrontPic[self.FromMessage];
        let mut nr: i32 =  index < 10000 ? self.game.Data.EventPicNr[index] : self.game.Data.HistoricalUnitObj[index - 10000].CommanderSpriteID;
        if (nr > -1)
        {
          let mut width1: i32 =  BitmapStore.GetWidth(nr);
          let mut height1: i32 =  BitmapStore.Getheight(nr);
          Rectangle rectangle;
          if (self.game.Data.RegimeObj[self.game.Data.Turn].MesStyle[self.FromMessage] == 2)
          {
            if (width1 > 160)
            {
              height1 =  Math.Round( height1 * (160.0 /  width1));
              width1 =  Math.Round( width1 * (160.0 /  width1));
            }
            if (height1 > 200)
            {
              width1 =  Math.Round( width1 * (200.0 /  height1));
              height1 =  Math.Round( height1 * (200.0 /  height1));
            }
            rectangle = Rectangle::new(85, 100, width1, height1);
            num1 = height1 + 100 + 20;
          }
          else
          {
            if (width1 > 724)
            {
              height1 =  Math.Round( height1 * (724.0 /  width1));
              width1 =  Math.Round( width1 * (724.0 /  width1));
            }
            if (height1 > 200)
            {
              width1 =  Math.Round( width1 * (200.0 /  height1));
              height1 =  Math.Round( height1 * (200.0 /  height1));
            }
            rectangle = Rectangle::new( Math.Round(405.0 -  width1 / 2.0), 20, width1, height1);
            num1 = height1 + 20 + 20;
          }
          if (self.game.Data.RegimeObj[self.game.Data.Turn].MessFrontPic[self.FromMessage] >= 10000)
          {
            DrawMod.DrawOfficer(g, self.game.Data.RegimeObj[self.game.Data.Turn].MessFrontPic[self.FromMessage] - 10000,  Math.Round(412.0 -  BitmapStore.GetWidth(nr) / 2.0), 70, BitmapStore.GetWidth(nr), BitmapStore.Getheight(nr));
            num1 += 50;
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
          }
        }
      }
      else
        num1 = 60;
      let mut num2: i32 =   Math.Round(Conversion.Int(32.5) -  num1 / 16.0);
      DrawMod.DrawPaperSheet( g, 55, num1 - 10, 690, 16 * (num2 - 1) + 20);
      let mut tsubpart1: SubPartClass =  new PaperAreaClass(self.game, 650, num2 - 2,  null, "Description", false, self.game.Data.RegimeObj[self.game.Data.Turn].MessString[self.FromMessage], self.game.VicColor8, tbackbitmap: ( self.OwnBitmap), bbx: 75, bby: num1);
      self.TAid = self.AddSubPart( tsubpart1, 75, num1, 650, 16 * (num2 - 2), 0);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("OK", 200, tBackbitmap: ( self.OwnBitmap), bbx: 300, bby: 540);
      self.okid = self.AddSubPart( tsubpart2, 300, 540, 200, 35, 1);
      if (Information.IsNothing( g))
        return;
      g.Dispose();
    }

    pub HandleKeyup: WindowReturnClass(nr: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr > 0)
        {
          windowReturnClass = self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.okid)] + 1, self.SubPartY[self.SubpartNr(self.okid)] + 1, 1);
          windowReturnClass.SetFlag(true);
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
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  self.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            let mut num: i32 =  self.SubPartID[index];
            if (num == self.TAid)
            {
              self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.okid)
            {
              if (self.FromMessage >= self.game.Data.RegimeObj[self.game.Data.Turn].MessCounter)
              {
                SoundMod.STopEventWave();
                if (self.game.EditObj.OrderType == 23)
                {
                  self.game.EditObj.PopupValue = 14;
                  windowReturnClass.AddCommand(5, 10);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              self += 1.FromMessage;
              if (Strings.Len(DrawMod.TGame.EditObj.CampaignRoomTitle) > 0)
                self.NewBackGroundAndClearAll(810, 610, DrawMod.TGame.BACKGROUND2MARC);
              else
                self.NewBackGroundAndClearAll(810, 610, self.tbacknr);
              self.ViewMessage();
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
