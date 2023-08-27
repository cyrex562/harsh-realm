// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TabMiniMapWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class TabMiniMapWindowClass2 : WindowClass
  {
     Info1Id: i32;
     info2id: i32;
     ShowString: String;
     DateTime ShowTime;
     w: i32;
     h: i32;
     CurrentView: i32;

    pub TabMiniMapWindowClass2(
       tGame: GameClass,
       WindowClass tLowerWindow,
       Rectangle tLowerRect,
      Rectangle trect)
      : base( tGame, trect.Width, trect.Height, 8)
    {
      self.w = trect.Width;
      self.h = trect.Height;
      self.LowerWindow = tLowerWindow;
      self.LowerRect = tLowerRect;
      self.dostuff();
    }

    pub fn DoRefresh() => self.dostuff();

    pub fn dostuff()
    {
      self.ClearMouse();
      self.NewBackGroundAndClearAll(self.w, self.h, -1);
      Rectangle trect = DrawMod.DrawBackTab(Graphics.FromImage((Image) self.OwnBitmap), self.w, self.h, "MINI", 7);
      self.AddMouse( trect, "CLOSE TAB", "Click here to close this tab. [ESC/F8]", 999);
      bool tpaintview;
      if (self.game.EditObj.Zoom == 0)
      {
        if ( (self.game.Data.MapObj[0].MapHeight + self.game.Data.MapObj[0].MapWidth) >  self.game.ScreenWidth / 52.0 +  self.game.ScreenHeight / 48.0)
          tpaintview = true;
      }
      else if (self.game.EditObj.Zoom == -1)
      {
        if ( (self.game.Data.MapObj[0].MapHeight + self.game.Data.MapObj[0].MapWidth) >  self.game.ScreenWidth / 27.0 +  self.game.ScreenHeight / 24.0)
          tpaintview = true;
      }
      else if (self.game.EditObj.Zoom == 1 &&  (self.game.Data.MapObj[0].MapHeight + self.game.Data.MapObj[0].MapWidth) >  self.game.ScreenWidth / 104.0 +  self.game.ScreenHeight / 96.0)
        tpaintview = true;
      if (self.Info1Id == 0)
      {
        let mut widthForMiniMap: i32 = DrawMod.GetWidthForMiniMap();
        let mut tsubpart: SubPartClass =  new MiniMapPartClass(DrawMod.TGame, tpaintview, widthForMiniMap, 200);
        self.Info1Id = self.AddSubPart( tsubpart, 16, 0, widthForMiniMap, 200, 0);
      }
      else
      {
        if (self.game.EditObj.Zoom != ((MiniMapPartClass) self.SubPartList[self.SubpartNr(self.Info1Id)]).tZoomLevel)
        {
          self.RemoveSubPart(self.Info1Id);
          let mut widthForMiniMap: i32 = DrawMod.GetWidthForMiniMap();
          let mut tsubpart: SubPartClass =  new MiniMapPartClass(DrawMod.TGame, tpaintview, widthForMiniMap, 200);
          self.Info1Id = self.AddSubPart( tsubpart, 16, 0, widthForMiniMap, 200, 0);
        }
        self.SubPartFlag[self.SubpartNr(self.Info1Id)] = true;
      }
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          return;
        }
      }
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (!(x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index]) || !(y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index]) || self.SubPartID[index] != self.Info1Id)
          ;
      }
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (self.MouseData[index] > 0 && x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height && self.MouseData[index] == 999)
        {
          self.game.EditObj.SetViewMode2 = 0;
          windowReturnClass.AddCommand(1, 9);
          windowReturnClass.AddCommand(7, 12);
          windowReturnClass.AddCommand(4, 67);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index] && self.SubPartID[index] == self.Info1Id)
        {
          self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
          windowReturnClass.AddCommand(4, 12);
          windowReturnClass.AddCommand(4, 68);
          windowReturnClass.AddCommand(4, 67);
          windowReturnClass.AddCommand(4, 9);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      if (x > 8 & x < self.w - 8 && y < self.h - 24)
        windowReturnClass.NoMouseClickBelow = true;
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
