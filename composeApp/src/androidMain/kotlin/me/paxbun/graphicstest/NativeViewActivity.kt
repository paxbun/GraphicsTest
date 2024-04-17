package me.paxbun.graphicstest

import RustNativeViewLib
import android.os.Bundle
import android.view.SurfaceHolder
import android.view.SurfaceView
import androidx.appcompat.app.AppCompatActivity
import graphics_test.RustNativeViewContext

class NativeViewActivity : AppCompatActivity() {

    private lateinit var surfaceView: SurfaceView
    private lateinit var nativeViewContext: RustNativeViewContext

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_native_view)



        surfaceView = findViewById(R.id.surfaceView)
        surfaceView.holder.addCallback(object : SurfaceHolder.Callback {
            override fun surfaceCreated(holder: SurfaceHolder) {
                nativeViewContext = RustNativeViewContext(
                    RustNativeViewLib.nativeViewAsNativeHandle(surfaceView.holder.surface).apply {
                        if (this == 0L) {
                            error("Could not create RustNativeViewContext")
                        }
                    },
                    resources.displayMetrics.density
                )
                nativeViewContext.render()
            }

            override fun surfaceChanged(
                holder: SurfaceHolder,
                format: Int,
                width: Int,
                height: Int
            ) {
                nativeViewContext.changeSize(width, height, resources.displayMetrics.density)
                nativeViewContext.render()
            }

            override fun surfaceDestroyed(holder: SurfaceHolder) {
                nativeViewContext.destroy()
            }
        })
    }
}